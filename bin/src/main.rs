mod parser;
mod transcript;
#[cfg(feature = "xs-rng")]
mod xs_rng;

use std::path::PathBuf;
use ark_ec::{AffineCurve, PairingEngine};
use ark_ff::{FftParameters, UniformRand, ToBytes, FromBytes, One};
use ark_bn254::{Fr, FrParameters, Bn254, G1Affine};
use ark_poly::{GeneralEvaluationDomain, univariate::DensePolynomial, EvaluationDomain};
use ark_poly_commit::{PolynomialCommitment, LabeledPolynomial};
use ark_serialize::*;
use clap::Parser;
use serde::{Serialize, Deserialize};
use rand::Rng;
use itertools::Itertools;
use posol_core::{balance_sum, tag, commitment::*};
use transcript::Transcript;
use parser::*;

#[derive(Debug, Parser)]
#[command(name = "Proof of Solvency", version = "0.0.1", about = "Proof of Solvency Simulator", long_about = "")]
enum Args {
    GenUsersData {
        #[arg(long = "users-size", value_parser)]
        users_size: u32,
        #[arg(long = "users-path")]
        users_path: PathBuf,
    },
    SetupKZG {
        #[arg(long = "ck-path")]
        ck_path: PathBuf,
        #[arg(long = "cvk-path")]
        cvk_path: PathBuf,
    },
    ProveAndCommit {
        #[arg(long = "ck-path")]
        ck_path: PathBuf,
        #[arg(long = "users-path")]
        users_path: PathBuf,
        #[arg(long = "witness-path")]
        witness_path: PathBuf,
    },
    SupplyWitness {
        #[arg(long = "user-index", value_parser)]
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
        Args::GenUsersData { users_size, users_path } => {
            #[cfg(feature = "xs-rng")]
            let rng = &mut xs_rng::get_xorshift_rng();
            #[cfg(not(feature = "xs-rng"))]
            let rng = &mut rand::thread_rng();

            let domain_size = max_domain_size() as u64;
            assert!(users_size as u64 <= domain_size);

            let users_info = (0..users_size)
                .into_iter()
                .map(|_| {
                    let mut id = [0u8; 32];
                    Fr::rand(rng).write(&mut id[..]).unwrap();
                    let balance = rng.gen_range(0..domain_size);

                    UserInfo { id, balance }
                })
                .collect_vec();
            json_to_file(&users_info, &users_path);
        }
        Args::SetupKZG { ck_path, cvk_path } => {
            #[cfg(feature = "xs-rng")]
            let rng = &mut xs_rng::get_xorshift_rng();
            #[cfg(not(feature = "xs-rng"))]
            let rng = &mut rand::thread_rng();

            let domain_size = max_domain_size();
            let max_degree = if cfg!(blinding) { domain_size + 3 } else { domain_size };

            let pp = KZG10::<Bn254>::setup(max_degree, None, rng).unwrap();
            let (ck, cvk) = KZG10::<Bn254>::trim(
                &pp,
                max_degree,
                0,
                None,
            ).unwrap();

            println!("G: x: {:#}", &cvk.g.x);
            println!("G: y: {:#}", &cvk.g.y);

            println!("H: x-c0: {:#}", &cvk.h.x.c0);
            println!("H: x-c1: {:#}", &cvk.h.x.c1);
            println!("H: y-c0: {:#}", &cvk.h.y.c0);
            println!("H: y-c1: {:#}", &cvk.h.y.c1);

            println!("Beta H: x-c0: {:#}", &cvk.beta_h.x.c0);
            println!("Beta H: x-c1: {:#}", &cvk.beta_h.x.c1);
            println!("Beta H: y-c0: {:#}", &cvk.beta_h.y.c0);
            println!("Beta H: y-c1: {:#}", &cvk.beta_h.y.c1);

            ser_to_file(&ck, &ck_path);
            ser_to_file(&cvk, &cvk_path);
        }
        Args::ProveAndCommit {
            ck_path,
            users_path,
            witness_path,
        } => {
            #[cfg(feature = "xs-rng")]
            let rng = &mut xs_rng::get_xorshift_rng();
            #[cfg(not(feature = "xs-rng"))]
            let rng = &mut rand::thread_rng();

            let n = max_domain_size();

            let ck: KZG10CommitterKey<Bn254> = deser_from_file(&ck_path);

            let users_data: Vec<UserInfo> = json_from_file(&users_path);
            assert!(users_data.len() <= n);
            let (tags, balances): (Vec<_>, Vec<_>) = users_data
                .iter()
                .map(|ui| (&ui.id[..], ui.balance))
                .unzip();

            // commit for tags first
            let (tag_commit, labeled_tag_poly) =
                tag::commit::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(&ck, n, &tags)
                    .expect("commit to tags failed");

            // // TODO: submit tag_commit on chain
            // println!("tag commit: {:#?}", &tag_commit);
                    
            // prove and commit for balances sum
            let (labeled_t_poly, t_commit) =
                balance_sum::precomute::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(&ck, n)
                    .expect("precomute for balances sum failed");

            println!("t commit: x: {:#}", &t_commit.0.x);
            println!("t commit: y: {:#}", &t_commit.0.y);

            let (_m, proof, labeled_b_poly) =
                balance_sum::prove::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>, Transcript, _>(
                    &ck,
                    n,
                    &labeled_t_poly,
                    &t_commit,
                    &balances,
                    rng,
                ).expect("prove for balances sum failed");

            let witness = Witness {
                tag_commit,
                labeled_tag_poly,
                b_commit: proof.b_commit,
                labeled_b_poly,
            };
            ser_to_file(&witness, &witness_path);
        }
        Args::SupplyWitness {
            user_index,
            ck_path,
            cvk_path,
            users_path,
            witness_path,
        } => {
            let n = max_domain_size();

            let ck: KZG10CommitterKey<Bn254> = deser_from_file(&ck_path);
            let cvk: KZG10VerifierKey<Bn254> = deser_from_file(&cvk_path);
            let witness: Witness = deser_from_file(&witness_path);
            let users_data: Vec<UserInfo> = json_from_file(&users_path);
            assert!(users_data.len() <= n);

            let tag_opening = tag::individual_open::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(
                &ck,
                n,
                user_index,
                &witness.labeled_tag_poly,
                &witness.tag_commit,
            ).expect("individual open for tag failed");

            let domain = GeneralEvaluationDomain::<Fr>::new(n).unwrap();
            kzg_check(
                &cvk,
                &witness.tag_commit,
                domain.element(user_index),
                Fr::read(&users_data[user_index].id[..]).unwrap(),
                &tag_opening,
            );

            let b_opening = balance_sum::individual_open::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(
                &ck,
                n,
                user_index,
                &witness.labeled_b_poly,
                &witness.b_commit,
            ).expect("individual open for balance failed");

            balance_sum::individual_verify::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(
                &cvk,
                n,
                user_index,
                users_data[user_index].balance,
                &witness.b_commit,
                &b_opening,
            ).expect("individual verify for balance failed");
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UserInfo {
    pub id: [u8; 32],
    pub balance: u64,
}

#[derive(Debug, CanonicalSerialize, CanonicalDeserialize)]
struct Witness {
    pub tag_commit: KZG10Commitment<Bn254>,
    pub labeled_tag_poly: LabeledPolynomial<Fr, DensePolynomial<Fr>>,
    pub b_commit: KZG10Commitment<Bn254>,
    pub labeled_b_poly: LabeledPolynomial<Fr, DensePolynomial<Fr>>,
}

fn max_domain_size() -> usize {
    let two_adicity = <FrParameters as FftParameters>::TWO_ADICITY;
    if cfg!(blinding) {
        (1usize << two_adicity) / 4
    } else {
        (1usize << two_adicity) / 2
    }
}

fn kzg_check(
    vk: &KZG10VerifierKey<Bn254>,
    comm: &KZG10Commitment<Bn254>,
    point: Fr,
    value: Fr,
    proof: &KZG10Proof<Bn254>,
) {
    let lhs = vk.g.mul(value) - comm.0.into_projective() - proof.w.mul(point);
    let a = (proof.w.clone().into(), vk.beta_h.clone().into());
    let b = (G1Affine::from(lhs).into(), vk.h.clone().into());
    
    assert!(Bn254::product_of_pairings([&a, &b]).is_one());
}
