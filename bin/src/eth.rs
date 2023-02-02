use ark_ff::{PrimeField, BigInteger};
use ark_poly::GeneralEvaluationDomain;
use ark_bn254::{Fr, G1Affine, Bn254};
use futures::executor::block_on;
use web3::{
    ethabi, Transport,
    contract::{Options, Contract},
    types::{U256, H256, Address},
};
use posol_core::{balance_sum, commitment::KZG10};

type Proof = balance_sum::Proof<Fr, GeneralEvaluationDomain<Fr>, KZG10<Bn254>>;

pub fn submit_sum_proof<T: Transport>(
    from: Address,
    options: Options,
    contract: &Contract<T>,
    proof: &Proof,
    balance_sum: &Fr,
) -> H256 {
    let call = async {
        contract
        .call(
            "verifyBalanceSum",
            (
                tokenize_sum_proof(proof),
                tokenize_fr(balance_sum),
            ),
            from,
            options,
        )
        .await
        .unwrap_or_else(|e| panic!("failed to submit proof: {:?}", e))
    };

    block_on(call)
}

// pub fn tokenize_bytes32(bytes: &[u8]) -> Token {
//     Token::FixedBytes(bytes.to_vec())
// }

fn tokenize_fr(fr: &Fr) -> ethabi::Token {
    let fr_repr = fr.into_repr().to_bytes_le();
    ethabi::Token::Tuple(vec![
        ethabi::Token::Uint(U256::from_little_endian(&fr_repr)),
    ])
}

fn tokenize_g1(g1: &G1Affine) -> ethabi::Token {
    let g1_x = g1.x.into_repr().to_bytes_le();
    let g1_y = g1.y.into_repr().to_bytes_le();
    ethabi::Token::Tuple(vec![
        ethabi::Token::Uint(U256::from_little_endian(&g1_x)),
        ethabi::Token::Uint(U256::from_little_endian(&g1_y)),
    ])
}

fn tokenize_sum_proof(proof: &Proof) -> ethabi::Token {
    ethabi::Token::Tuple(vec![
        tokenize_fr(&proof.evaluations.b),
        tokenize_fr(&proof.evaluations.t),
        tokenize_fr(&proof.evaluations.h1),
        tokenize_fr(&proof.evaluations.h2),
        tokenize_fr(&proof.evaluations.s_next),
        tokenize_fr(&proof.evaluations.z_next),
        tokenize_fr(&proof.evaluations.h1_next),
        tokenize_fr(&proof.evaluations.h2_next),
        tokenize_g1(&proof.b_commit.0),
        tokenize_g1(&proof.s_commit.0),
        tokenize_g1(&proof.h1_commit.0),
        tokenize_g1(&proof.h2_commit.0),
        tokenize_g1(&proof.z_commit.0),
        tokenize_g1(&proof.q1_commit.0),
        tokenize_g1(&proof.q2_commit.0),
        tokenize_g1(&proof.w_opening.w),
        tokenize_g1(&proof.sw_opening.w),
    ])
}
