mod eth;
mod parser;
mod transcript;
#[cfg(feature = "xs-rng")]
mod xs_rng;

use std::path::PathBuf;
use ark_ff::{UniformRand, ToBytes};
use ark_bn254::{Fr, Bn254};
use ark_poly::{GeneralEvaluationDomain, univariate::DensePolynomial, EvaluationDomain};
use ark_poly_commit::{PolynomialCommitment, LabeledPolynomial};
use ark_serialize::*;
use clap::Parser;
use serde::{Serialize, Deserialize};
use rand::Rng;
use itertools::Itertools;
use web3::{
    ethabi,
    api::{Eth, Namespace},
    types::Address,
    contract::Contract,
    transports::Http,
};
use posol_core::{balance_sum, tag, util::EvaluationDomainExt, commitment::*};
use transcript::Transcript;
use parser::*;

#[derive(Debug, Parser)]
#[command(name = "Proof of Solvency", version = "0.0.1", about = "Proof of Solvency Simulator", long_about = "")]
enum Args {
    GenUsers {
        #[arg(long = "domain-size", default_value = "134217728")]
        domain_size: usize,
        #[arg(long = "users-size")]
        users_size: usize,
        #[arg(long = "users-path")]
        users_path: PathBuf,
    },
    SetupKZG {
        #[arg(long = "domain-size", default_value = "134217728")]
        domain_size: usize,
        #[arg(long = "ck-path")]
        ck_path: PathBuf,
        #[arg(long = "cvk-path")]
        cvk_path: PathBuf,
    },
    PrintParams {
        #[arg(long = "domain-size", default_value = "134217728")]
        domain_size: usize,
        #[arg(long = "ck-path")]
        ck_path: PathBuf,
        #[arg(long = "cvk-path")]
        cvk_path: PathBuf,
    },
    ProveAndCommit {
        #[arg(long = "domain-size", default_value = "134217728")]
        domain_size: usize,
        #[arg(long = "ck-path")]
        ck_path: PathBuf,
        #[arg(long = "cvk-path")]
        cvk_path: PathBuf,
        #[arg(long = "users-path")]
        users_path: PathBuf,
        #[arg(long = "witness-path")]
        witness_path: PathBuf,
        #[arg(long = "eth-path")]
        eth_path: Option<PathBuf>,
    },
    SupplyWitness {
        #[arg(long = "domain-size", default_value = "134217728")]
        domain_size: usize,
        #[arg(long = "user-index")]
        user_index: usize,
        #[arg(long = "ck-path")]
        ck_path: PathBuf,
        #[arg(long = "cvk-path")]
        cvk_path: PathBuf,
        #[arg(long = "users-path")]
        users_path: PathBuf,
        #[arg(long = "witness-path")]
        witness_path: PathBuf,
    }
}

fn main() {
    let args = Args::parse();
    match args {
        Args::GenUsers {
            domain_size,
            users_size,
            users_path,
        } => {
            #[cfg(feature = "xs-rng")]
            let rng = &mut xs_rng::get_xorshift_rng();
            #[cfg(not(feature = "xs-rng"))]
            let rng = &mut rand::thread_rng();

            assert!(users_size <= domain_size);

            let users_info = (0..users_size)
                .into_iter()
                .map(|_| {
                    let mut tag = [0u8; 32];
                    Fr::rand(rng).write(&mut tag[..]).unwrap();
                    let balance = rng.gen_range(0..domain_size as u64);

                    UserInfo { tag, balance }
                })
                .collect_vec();
            json_to_file(&users_info, &users_path);
        }
        Args::SetupKZG {
            domain_size,
            ck_path,
            cvk_path,
        } => {
            #[cfg(feature = "xs-rng")]
            let rng = &mut xs_rng::get_xorshift_rng();
            #[cfg(not(feature = "xs-rng"))]
            let rng = &mut rand::thread_rng();

            let max_degree = if cfg!(blinding) { domain_size + 3 } else { domain_size };
            let pp = KZG10::<Bn254>::setup(max_degree, None, rng)
                .expect("invalid max degree");
            let (ck, cvk) = KZG10::<Bn254>::trim(
                &pp,
                max_degree,
                0,
                None,
            ).unwrap();

            ser_to_file(&ck, &ck_path);
            ser_to_file(&cvk, &cvk_path);
        }
        Args::PrintParams {
            domain_size,
            ck_path,
            cvk_path,
        } => {
            let ck: KZG10CommitterKey<Bn254> = deser_from_file(&ck_path);
            let cvk: KZG10VerifierKey<Bn254> = deser_from_file(&cvk_path);

            let domain = GeneralEvaluationDomain::<Fr>::new(domain_size)
                .expect("invalid domain size");

            let (_, t_commit) =
                balance_sum::precompute::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(&ck, domain_size)
                    .expect("precompute failed");

            println!("domain group gen: {}", eth::Param::Fr(domain.group_gen()));
            println!("domain group gen inv: {}", eth::Param::Fr(domain.group_gen_inv()));

            println!("G: {}", eth::Param::G1Affine(cvk.g));
            println!("H: {}", eth::Param::G2Affine(cvk.h));
            println!("Beta H: {}", eth::Param::G2Affine(cvk.beta_h));

            println!("t commit: {}", eth::Param::G1Affine(t_commit.0));
        }
        Args::ProveAndCommit {
            domain_size,
            ck_path,
            cvk_path,
            users_path,
            witness_path,
            eth_path,
        } => {
            #[cfg(feature = "xs-rng")]
            let rng = &mut xs_rng::get_xorshift_rng();
            #[cfg(not(feature = "xs-rng"))]
            let rng = &mut rand::thread_rng();

            let ck: KZG10CommitterKey<Bn254> = deser_from_file(&ck_path);
            let cvk: KZG10VerifierKey<Bn254> = deser_from_file(&cvk_path);
            let users_data: Vec<UserInfo> = json_from_file(&users_path);
            assert!(users_data.len() <= domain_size);
            let (tags, balances): (Vec<_>, Vec<_>) = users_data
                .iter()
                .map(|ui| (&ui.tag[..], ui.balance))
                .unzip();

            let (labeled_t_poly, t_commit) =
                balance_sum::precompute::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(&ck, domain_size)
                    .expect("precompute failed");

            // commit for tags first
            let (tag_commit, labeled_tag_poly) =
                tag::commit::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(
                    &ck,
                    domain_size,
                    &tags,
                )
                .expect("commit to tags failed");
                    
            // prove and commit for balances sum
            let (m, proof, labeled_b_poly) =
                balance_sum::prove::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>, Transcript, _>(
                    &ck,
                    domain_size,
                    &labeled_t_poly,
                    &t_commit,
                    &balances,
                    rng,
                ).expect("prove for balances sum failed");

            proof.verify::<Transcript>(&cvk, domain_size, &t_commit, m)
                .expect("proof verification failed");

            let witness = Witness {
                tag_commit: tag_commit.clone(),
                labeled_tag_poly,
                b_commit: proof.b_commit.clone(),
                labeled_b_poly,
            };
            ser_to_file(&witness, &witness_path);

            let tag_commit = eth::Param::G1Affine(tag_commit.0);
            let proof = eth::Param::Proof(proof);
            let m = eth::Param::Fr(m);
            if let Some(eth_path) = eth_path {
                let eth_config: EthConfig = json_from_file(&eth_path);
                let transport = Http::new(&eth_config.url).expect("failed to connect to eth network");
                let contract = Contract::new(
                    Eth::new(transport),
                    eth_config.contract,
                    eth_config.abi,
                );
                // submit `m` and `proof` on chain.
                println!("submitting proof to eth network...");
                let tx_hash = eth::call_contract(
                    eth_config.sender,
                    Default::default(),
                    &contract,
                    "verifyBalanceSum",
                    &[proof, m],
                );
                println!("transaction hash: {:x}", tx_hash);
            } else {
                println!("tag commitment: {}", tag_commit);
                println!("proof: {}", proof);
                println!("balance sum: {}", m);
            }
        }
        Args::SupplyWitness {
            domain_size,
            user_index,
            ck_path,
            cvk_path,
            users_path,
            witness_path,
        } => {
            let ck: KZG10CommitterKey<Bn254> = deser_from_file(&ck_path);
            let cvk: KZG10VerifierKey<Bn254> = deser_from_file(&cvk_path);
            let witness: Witness = deser_from_file(&witness_path);
            let users_data: Vec<UserInfo> = json_from_file(&users_path);
            assert!(users_data.len() <= domain_size);

            let tag_opening = tag::individual_open::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(
                &ck,
                domain_size,
                user_index,
                &witness.labeled_tag_poly,
                &witness.tag_commit,
            ).expect("individual open for tag failed");

            tag::individual_verify::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(
                &cvk,
                domain_size,
                user_index,
                &users_data[user_index].tag,
                &witness.tag_commit,
                &tag_opening,
            ).expect("individual verify for tag failed");

            let tag_opening = eth::Param::G1Affine(tag_opening.w);
            println!("tag opening: {}", tag_opening);

            let b_opening = balance_sum::individual_open::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(
                &ck,
                domain_size,
                user_index,
                &witness.labeled_b_poly,
                &witness.b_commit,
            ).expect("individual open for balance failed");

            balance_sum::individual_verify::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(
                &cvk,
                domain_size,
                user_index,
                users_data[user_index].balance,
                &witness.b_commit,
                &b_opening,
            ).expect("individual verify for balance failed");

            let b_opening = eth::Param::G1Affine(b_opening.w);
            println!("balance opening: {}", b_opening);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UserInfo {
    pub tag: [u8; 32],
    pub balance: u64,
}

#[derive(Debug, CanonicalSerialize, CanonicalDeserialize)]
struct Witness {
    pub tag_commit: KZG10Commitment<Bn254>,
    pub labeled_tag_poly: LabeledPolynomial<Fr, DensePolynomial<Fr>>,
    pub b_commit: KZG10Commitment<Bn254>,
    pub labeled_b_poly: LabeledPolynomial<Fr, DensePolynomial<Fr>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EthConfig {
    pub url: String,
    pub sender: Address,
    pub contract: Address,
    pub abi: ethabi::Contract,
}

// fn max_domain_size() -> usize {
//     let two_adicity = <FrParameters as FftParameters>::TWO_ADICITY;
//     if cfg!(blinding) {
//         (1usize << two_adicity) / 4
//     } else {
//         (1usize << two_adicity) / 2
//     }
// }
