use ark_ff::{PrimeField, BigInteger};
use ark_poly::GeneralEvaluationDomain;
use ark_bn254::{Fr, G1Affine, G2Affine, Bn254};
use futures::executor::block_on;
use itertools::Itertools;
use web3::{
    Transport,
    ethabi::Token,
    contract::{Options, Contract},
    types::{U256, H256, Address},
};
use posol_core::{balance_sum, commitment::KZG10};

type Proof = balance_sum::Proof<Fr, GeneralEvaluationDomain<Fr>, KZG10<Bn254>>;

fn fmt_fr(f: &mut std::fmt::Formatter<'_>, fr: &Fr) -> std::fmt::Result {
    write!(f, "{{ value: 0x{} }}", hex::encode(fr.into_repr().to_bytes_be()))
}

fn fmt_g1_affine(f: &mut std::fmt::Formatter<'_>, g1: &G1Affine) -> std::fmt::Result {
    write!(
        f,
        "{{ x: 0x{}, y: 0x{}}}",
        hex::encode(g1.x.into_repr().to_bytes_be()),
        hex::encode(g1.y.into_repr().to_bytes_be()),
    )
}

fn fmt_g2_affine(f: &mut std::fmt::Formatter<'_>, g2: &G2Affine) -> std::fmt::Result {
    write!(
        f,
        "{{ x: [0x{}, 0x{}], y: [0x{}, 0x{}] }}",
        hex::encode(g2.x.c1.into_repr().to_bytes_be()),
        hex::encode(g2.x.c0.into_repr().to_bytes_be()),
        hex::encode(g2.y.c1.into_repr().to_bytes_be()),
        hex::encode(g2.y.c0.into_repr().to_bytes_be()),
    )
}

pub enum Param {
    Fr(Fr),
    G1Affine(G1Affine),
    G2Affine(G2Affine),
    Proof(Proof),
}

impl std::fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Param::Fr(fr) => fmt_fr(f, fr),
            Param::G1Affine(g1) => fmt_g1_affine(f, g1),
            Param::G2Affine(g2) => fmt_g2_affine(f, g2),
            Param::Proof(proof) => {
                macro_rules! with_fix {
                    ($func:ident, $prefix:expr, $val:expr, $suffix:expr) => {
                        write!(f, $prefix)?;
                        $func(f, $val)?;
                        write!(f, $suffix)?;
                    };
                }
                write!(f, "{{\n")?;
                with_fix!(fmt_fr, "\tb: ", &proof.evaluations.b, ",\n");
                with_fix!(fmt_fr, "\tt: ", &proof.evaluations.t, ",\n");
                with_fix!(fmt_fr, "\th1: ", &proof.evaluations.h1, ",\n");
                with_fix!(fmt_fr, "\th2: ", &proof.evaluations.h2, ",\n");
                with_fix!(fmt_fr, "\tsNext: ", &proof.evaluations.s_next, ",\n");
                with_fix!(fmt_fr, "\tzNext: ", &proof.evaluations.z_next, ",\n");
                with_fix!(fmt_fr, "\th1Next: ", &proof.evaluations.h1_next, ",\n");
                with_fix!(fmt_fr, "\th2Next: ", &proof.evaluations.h2_next, ",\n");
                with_fix!(fmt_g1_affine, "\tbCommit: ", &proof.b_commit.0, ",\n");
                with_fix!(fmt_g1_affine, "\tsCommit: ", &proof.s_commit.0, ",\n");
                with_fix!(fmt_g1_affine, "\th1Commit: ", &proof.h1_commit.0, ",\n");
                with_fix!(fmt_g1_affine, "\th2Commit: ", &proof.h2_commit.0, ",\n");
                with_fix!(fmt_g1_affine, "\tzCommit: ", &proof.z_commit.0, ",\n");
                with_fix!(fmt_g1_affine, "\tq1Commit: ", &proof.q1_commit.0, ",\n");
                with_fix!(fmt_g1_affine, "\tq2Commit: ", &proof.q2_commit.0, ",\n");
                with_fix!(fmt_g1_affine, "\topening1: ", &proof.w_opening.w, ",\n");
                with_fix!(fmt_g1_affine, "\topening2: ", &proof.sw_opening.w, "\n");
                write!(f, "}}")
            }
        }
    }
}

impl<'a> Into<Token> for &'a Param {
    fn into(self) -> Token {
        match self {
            Param::Fr(fr) => tokenize_fr(fr),
            Param::G1Affine(g1) => tokenize_g1(g1),
            Param::G2Affine(g2) => tokenize_g2(g2),
            Param::Proof(proof) => tokenize_sum_proof(proof),
        }
    }
}

pub fn call_contract<T: Transport>(
    from: Address,
    options: Options,
    contract: &Contract<T>,
    func_name: &str,
    params: &[Param],
) -> H256 {
    let params = params.iter().map(Into::into).collect_vec();
    let call = async {
        contract
            .call(
                func_name,
                &params[..],
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

fn tokenize_fr(fr: &Fr) -> Token {
    let fr_repr = fr.into_repr().to_bytes_le();
    Token::Tuple(vec![
        Token::Uint(U256::from_little_endian(&fr_repr)),
    ])
}

fn tokenize_g1(g1: &G1Affine) -> Token {
    let x = g1.x.into_repr().to_bytes_le();
    let y = g1.y.into_repr().to_bytes_le();
    Token::Tuple(vec![
        Token::Uint(U256::from_little_endian(&x)),
        Token::Uint(U256::from_little_endian(&y)),
    ])
}

fn tokenize_g2(g2: &G2Affine) -> Token {
    let x_c0 = g2.x.c0.into_repr().to_bytes_le();
    let x_c1 = g2.x.c1.into_repr().to_bytes_le();
    let y_c0 = g2.y.c0.into_repr().to_bytes_le();
    let y_c1 = g2.y.c1.into_repr().to_bytes_le();
    Token::Tuple(vec![
        Token::FixedArray(vec![
            Token::Uint(U256::from_little_endian(&x_c1)),
            Token::Uint(U256::from_little_endian(&x_c0)),
        ]),
        Token::FixedArray(vec![
            Token::Uint(U256::from_little_endian(&y_c1)),
            Token::Uint(U256::from_little_endian(&y_c0)),
        ]),
    ])
}

fn tokenize_sum_proof(proof: &Proof) -> Token {
    Token::Tuple(vec![
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
