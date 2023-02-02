
mod linear_poly;
mod quotient_poly;
mod proof;
mod transcript;

pub use proof::*;
pub use transcript::*;

use anyhow::{anyhow, Result};
use ark_std::{collections::HashMap, start_timer, end_timer};
use ark_ff::{FftField, Field};
use ark_poly::{EvaluationDomain, univariate::DensePolynomial, UVPolynomial};
use ark_poly_commit::{PCRandomness, LabeledPolynomial};
use itertools::Itertools;
use rand_core::{CryptoRng, RngCore};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::{
    util::{EvaluationDomainExt, poly_from_evals_ref, poly_from_evals},
    commitment::HomomorphicCommitment,
    label_polynomial, label_commitment,
};

pub fn precompute<F, D, PC>(
    ck: &PC::CommitterKey,
    n: usize,
) -> Result<(LabeledPolynomial<F, DensePolynomial<F>>, PC::Commitment)>
where
    F: FftField,
    D: EvaluationDomain<F>,
    PC: HomomorphicCommitment<F>,
{
    assert!(n.is_power_of_two());

    let domain = D::new(n)
        .ok_or(anyhow!(
            "log size of group: {}, 2-adicity: {}",
            n.trailing_zeros(),
            <F::FftParams as ark_ff::FftParameters>::TWO_ADICITY,
        ))?;

    let timer = start_timer!(|| "Balance Sum: Precomputing");

    // Precompute t(X).
    let t_evals = (0..n).into_iter().map(|i| F::from(i as u64)).collect_vec();
    let t_poly = poly_from_evals(&domain, t_evals);
    let labeled_t_poly = label_polynomial!(t_poly);

    // Commit to t(X).
    let (labeled_t_commit, _) =
        PC::commit(ck, vec![&labeled_t_poly], None)
            .map_err(|e| anyhow!("commit to t(X) failed: {}", e))?;

    end_timer!(timer);

    Ok((
        labeled_t_poly,
        labeled_t_commit[0].commitment().clone(),
    ))
}

pub fn prove<F, D, PC, T, R>(
    ck: &PC::CommitterKey,
    n: usize,
    labeled_t_poly: &LabeledPolynomial<F, DensePolynomial<F>>,
    t_commit: &PC::Commitment,
    balances: &[u64],
    rng: &mut R,
) -> Result<(F, Proof<F, D, PC>, LabeledPolynomial<F, DensePolynomial<F>>)>
where
    F: FftField,
    D: EvaluationDomain<F> + EvaluationDomainExt<F>,
    PC: HomomorphicCommitment<F>,
    T: TranscriptProtocol<F, PC::Commitment>,
    R: CryptoRng + RngCore,
{
    assert!(n.is_power_of_two());
    assert!(balances.len() <= n);

    for &balance in balances {
        assert!(balance < n as u64);
    }

    let domain = D::new(n)
        .ok_or(anyhow!(
            "log size of group: {}, 2-adicity: {}",
            n.trailing_zeros(),
            <F::FftParams as ark_ff::FftParameters>::TWO_ADICITY,
        ))?;

    let timer = start_timer!(|| "Balance Sum: Proving");

    let transcript = &mut T::new("Proof of Balance Sum");
    transcript.append_u64("n", n as u64);

    // Compute balances vector `B`.
    let mut b_evals = balances.iter().map(|&b| F::from(b)).collect_vec();
    b_evals.resize(n, F::zero());

    // Compute aux vector `S`.
    let s_evals = generate_s_evals(&b_evals);
    let m = s_evals[0];

    // Add public input to transcript.
    transcript.append_scalar("m", &m);

    // Compute polynomials B(X).
    let mut b_poly = poly_from_evals_ref(&domain, &b_evals);
    if cfg!(blinding) {
        add_blinders_to_poly(rng, 2, &mut b_poly);
    }
    let labeled_b_poly = label_polynomial!(b_poly);

    // Compute aux polynomial S(X).
    let mut s_poly = poly_from_evals(&domain, s_evals);
    if cfg!(blinding) {
        add_blinders_to_poly(rng, 3, &mut s_poly);
    }
    let labeled_s_poly = label_polynomial!(s_poly);

    // Compute polynomials h1(X) and h2(X).
    let (h1_evals, h2_evals) = generate_h_evals(&b_evals);
    let mut h1_poly = poly_from_evals_ref(&domain, &h1_evals);
    if cfg!(blinding) {
        add_blinders_to_poly(rng, 3, &mut h1_poly);
    }
    let labeled_h1_poly = label_polynomial!(h1_poly);

    let mut h2_poly = poly_from_evals_ref(&domain, &h2_evals);
    if cfg!(blinding) {
        add_blinders_to_poly(rng, 3, &mut h2_poly);
    }
    let labeled_h2_poly = label_polynomial!(h2_poly);

    // Commit to B(X), S(X), h1(X), h2(X)
    let (labeled_bsh_commits, _) =
        PC::commit(ck, vec![
            &labeled_b_poly,
            &labeled_s_poly,
            &labeled_h1_poly,
            &labeled_h2_poly,
        ], None)
        .map_err(|e| {
            anyhow!("commit to B(x), S(X), h1(X), h2(X) failed: {}", e)
        })?;

    // Add commitments to transcript.
    transcript.append_commitment("b_commit", labeled_bsh_commits[0].commitment());
    transcript.append_commitment("s_commit", labeled_bsh_commits[1].commitment());
    transcript.append_commitment("h1_commit", labeled_bsh_commits[2].commitment());
    transcript.append_commitment("h2_commit", labeled_bsh_commits[3].commitment());

    // Fiat-Shamir challenge
    let gamma = transcript.challenge_scalar("gamma");

    // Compute polynomial z(X)
    let z_evals = generate_z_evals(gamma, &b_evals, &h1_evals, &h2_evals);
    drop(b_evals);
    drop(h1_evals);
    drop(h2_evals);
    let mut z_poly = poly_from_evals(&domain, z_evals);
    if cfg!(blinding) {
        add_blinders_to_poly(rng, 3, &mut z_poly);
    }
    let labeled_z_poly = label_polynomial!(z_poly);

    // Commit to z(X).
    let (labeled_z_commit, _) =
        PC::commit(ck, vec![&labeled_z_poly], None)
            .map_err(|e| anyhow!("commit to z(X) failed: {}", e))?;
    
    // Add commitment to transcript.
    transcript.append_commitment("z_commit", labeled_z_commit[0].commitment());

    // Fiat-Shamir challenge
    let delta = transcript.challenge_scalar("delta");

    let q_poly = quotient_poly::compute(
        &domain,
        m,
        gamma,
        delta,
        labeled_t_poly.polynomial(),
        labeled_b_poly.polynomial(),
        labeled_s_poly.polynomial(),
        labeled_h1_poly.polynomial(),
        labeled_h2_poly.polynomial(),
        labeled_z_poly.polynomial(),
    )?;
    
    // Split quotient polynomials.
    let split = if cfg!(blinding) { n + 3 } else { n };
    let mut q1_poly = DensePolynomial::from_coefficients_slice(&q_poly[..split]);
    let mut q2_poly = DensePolynomial::from_coefficients_slice(&q_poly[split..]);
    if cfg!(blinding) {
        // Add blinding factors for quotient polynomials.
        let e0 = F::rand(rng);
        q1_poly.coeffs.push(e0);
        q2_poly.coeffs[0] -= e0;
    }
    let labeled_q1_poly = label_polynomial!(q1_poly);
    let labeled_q2_poly = label_polynomial!(q2_poly);
    
    // Commit to quotient polynomials.
    let (labeled_q_commits, _) =
        PC::commit(ck, vec![&labeled_q1_poly, &labeled_q2_poly], None)
            .map_err(|e| anyhow!("commit to q1(X), q2(X) failed: {}", e))?;

    // Add commitments to transcript.
    transcript.append_commitment("q1_commit", labeled_q_commits[0].commitment());
    transcript.append_commitment("q2_commit", labeled_q_commits[1].commitment());

    // Compute evaluation point challenge `z`.
    let z = transcript.challenge_scalar("z");

    let (r_poly, evaluations) = linear_poly::compute(
        &domain,
        gamma,
        delta,
        z,
        labeled_t_poly.polynomial(),
        labeled_b_poly.polynomial(),
        labeled_s_poly.polynomial(),
        labeled_h1_poly.polynomial(),
        labeled_h2_poly.polynomial(),
        labeled_z_poly.polynomial(),
        labeled_q1_poly.polynomial(),
        labeled_q2_poly.polynomial(),
    );
    drop(labeled_q1_poly);
    drop(labeled_q2_poly);
    let labeled_r_poly = label_polynomial!(r_poly);

    transcript.append_scalar("t_eval", &evaluations.t);
    transcript.append_scalar("b_eval", &evaluations.b);
    transcript.append_scalar("h1_eval", &evaluations.h1);
    transcript.append_scalar("h2_eval", &evaluations.h2);
    transcript.append_scalar("s_next_eval", &evaluations.s_next);
    transcript.append_scalar("h1_next_eval", &evaluations.h1_next);
    transcript.append_scalar("h2_next_eval", &evaluations.h2_next);
    transcript.append_scalar("z_next_eval", &evaluations.z_next);

    // Compute opening point challenge `eta`.
    let eta = transcript.challenge_scalar("eta");

    // Commit to linear polynomial.
    let (labeled_r_commit, _) =
        PC::commit(ck, vec![&labeled_r_poly], None)
            .map_err(|e| anyhow!("commit to r(X) failed: {}", e))?;

    let labeled_t_commit = label_commitment!(t_commit);
    let randomness = <PC::Randomness as PCRandomness>::empty();
    // Compute opening proofs.
    let w_opening = PC::open(
        ck,
        vec![
            &labeled_r_poly,
            &labeled_t_poly,
            &labeled_b_poly,
            &labeled_h1_poly,
            &labeled_h2_poly,
        ],
        vec![
            &labeled_r_commit[0],
            &labeled_t_commit,
            &labeled_bsh_commits[0],
            &labeled_bsh_commits[2],
            &labeled_bsh_commits[3],
        ],
        &z,
        eta,
        vec![&randomness, &randomness, &randomness, &randomness, &randomness],
        None,
    )
    .map_err(|e| anyhow!("open W(X) failed: {}", e))?;
    drop(labeled_r_poly);

    let sw_opening = PC::open(
        ck,
        vec![
            &labeled_s_poly,
            &labeled_h1_poly,
            &labeled_h2_poly,
            &labeled_z_poly,
        ],
        vec![
            &labeled_bsh_commits[1],
            &labeled_bsh_commits[2],
            &labeled_bsh_commits[3],
            &labeled_z_commit[0],
        ],
        &(z * domain.group_gen()),
        eta,
        vec![&randomness, &randomness, &randomness, &randomness],
        None,
    )
    .map_err(|e| anyhow!("open W_next(X) failed: {}", e))?;

    let proof = Proof {
        b_commit: labeled_bsh_commits[0].commitment().clone(),
        s_commit: labeled_bsh_commits[1].commitment().clone(),
        h1_commit: labeled_bsh_commits[2].commitment().clone(),
        h2_commit: labeled_bsh_commits[3].commitment().clone(),
        z_commit: labeled_z_commit[0].commitment().clone(),
        q1_commit: labeled_q_commits[0].commitment().clone(),
        q2_commit: labeled_q_commits[1].commitment().clone(),
        w_opening,
        sw_opening,
        evaluations,
        _p: std::marker::PhantomData,
    };

    end_timer!(timer);

    Ok((m, proof, labeled_b_poly))
}

pub fn individual_open<F, D, PC>(
    ck: &PC::CommitterKey,
    n: usize,
    i: usize,
    labeled_b_poly: &LabeledPolynomial<F, DensePolynomial<F>>,
    b_commit: &PC::Commitment,
) -> Result<PC::Proof>
where
    F: FftField,
    D: EvaluationDomain<F>,
    PC: HomomorphicCommitment<F>,
{
    assert!(n.is_power_of_two());

    let domain = D::new(n)
        .ok_or(anyhow!(
            "log size of group: {}, 2-adicity: {}",
            n.trailing_zeros(),
            <F::FftParams as ark_ff::FftParameters>::TWO_ADICITY,
        ))?;

    let timer = start_timer!(|| "Balance: Individual Opening");

    let point = domain.element(i);
    let labeled_b_commit = label_commitment!(b_commit);
    let randomness = <PC::Randomness as PCRandomness>::empty();
    let proof = PC::open(
        ck,
        vec![labeled_b_poly],
        vec![&labeled_b_commit],
        &point,
        F::one(),
        vec![&randomness],
        None,
    )
    .map_err(|e| anyhow!("open B(X) failed: {}", e))?;

    end_timer!(timer);

    Ok(proof)
}

pub fn individual_verify<F, D, PC>(
    vk: &PC::VerifierKey,
    n: usize,
    i: usize,
    balance: u64,
    b_commit: &PC::Commitment,
    proof: &PC::Proof,
) -> Result<()>
where
    F: FftField,
    D: EvaluationDomain<F>,
    PC: HomomorphicCommitment<F>,
{
    assert!(n.is_power_of_two());

    let domain = D::new(n)
        .ok_or(anyhow!(
            "log size of group: {}, 2-adicity: {}",
            n.trailing_zeros(),
            <F::FftParams as ark_ff::FftParameters>::TWO_ADICITY,
        ))?;

    let timer = start_timer!(|| "Balance: Individual Verifying");

    let point = domain.element(i);
    let evaluation = F::from(balance);
    let labeled_b_commit = label_commitment!(b_commit);
    match PC::check(
        vk,
        vec![&labeled_b_commit],
        &point,
        vec![evaluation],
        &proof,
        F::one(),
        None,
    ) {
        Ok(true) => {
            end_timer!(timer);
            Ok(())
        }
        Ok(false) => Err(anyhow!("individual balance verification failed")),
        Err(e) => Err(anyhow!("check opening proof error: {}", e)),
    }
}

/// Add blinding factors to polynomial.
fn add_blinders_to_poly<F, R>(rng: &mut R, k: usize, poly: &mut DensePolynomial<F>)
where
    F: Field,
    R: RngCore + CryptoRng,
{
    let blinders = (0..k).into_iter().map(|_| F::rand(rng)).collect_vec();
    poly.coeffs.extend_from_slice(&blinders);
    
    ark_std::cfg_iter_mut!(poly.coeffs)
        .zip(blinders)
        .for_each(|(coeff, blinder)| coeff.sub_assign(blinder));
}

fn generate_s_evals<F: Field>(b_evals: &[F]) -> Vec<F> {
    // Compute aux vector `S`.
    let mut s_evals = Vec::with_capacity(b_evals.len());
    let mut sum = F::zero();
    for b in b_evals {
        sum += b;
        s_evals.push(sum);
    }

    let m = s_evals.pop().unwrap();
    s_evals.insert(0, m);

    s_evals
}

fn generate_h_evals<F: Field>(b_evals: &[F]) -> (Vec<F>, Vec<F>) {
    let n = b_evals.len();

    let mut counter = HashMap::with_capacity(n);
    for balance in b_evals {
        if let Some(num) = counter.get_mut(balance) {
            *num += 1usize;
        } else {
            counter.insert(*balance, 1usize);
        }
    }

    let mut current = F::zero();
    let mut h = Vec::with_capacity(2 * n);
    loop {
        if let Some(&num) = counter.get(&current) {
            h.extend(vec![current; num + 1]);
        } else {
            h.push(current);
        }
        current += F::one();

        if h.len() >= 2 * n {
            break;
        }
    }

    // Sanity check
    assert_eq!(current, F::from(n as u64));

    let (h1, h2) = h.split_at(n);
    (h1.to_vec(), h2.to_vec())
}

fn generate_z_evals<F: Field>(
    gamma: F,
    b_evals: &[F],
    h1_evals: &[F],
    h2_evals: &[F],
) -> Vec<F> {
    let n = b_evals.len();
    assert_eq!(h1_evals.len(), n);
    assert_eq!(h2_evals.len(), n);

    let mut product = F::one();
    let mut z_evals = Vec::with_capacity(b_evals.len());
    z_evals.push(product);
    for i in 0..(n - 1) {
        let numerator = (gamma + b_evals[i]) * (gamma + F::from(i as u64));
        let denominator = (gamma + h1_evals[i]) * (gamma + h2_evals[i]);

        product *= numerator * denominator.inverse().unwrap();
        z_evals.push(product);
    }

    z_evals
}

#[cfg(test)]
mod test {
    use ark_ff::UniformRand;
    use ark_poly::{GeneralEvaluationDomain, EvaluationDomain, Polynomial};
    use ark_poly_commit::PolynomialCommitment;
    use ark_std::{test_rng, rand::Rng};
    use ark_bn254::{Bn254, Fr};
    use itertools::Itertools;
    use num_traits::{Zero, One};
    
    use crate::{commitment::KZG10, util::poly_from_evals_ref};
    use super::{*, transcript::MerlinTranscript};

    #[test]
    fn test_add_blinders_to_poly() {
        let rng = &mut test_rng();
        // 8 degree poly
        let domain = GeneralEvaluationDomain::new(8).unwrap();
        let evals = (0..8).into_iter().map(|_| Fr::rand(rng)).collect_vec();
        let poly = poly_from_evals_ref(&domain, &evals);

        // add 1 blinder
        let mut poly_1 = poly.clone();
        add_blinders_to_poly(rng, 1, &mut poly_1);
        for (ele, expect) in domain.elements().zip(evals.iter()) {
            let res = poly_1.evaluate(&ele);
            assert_eq!(&res, expect);
        }
        // add 2 blinders
        let mut poly_2 = poly.clone();
        add_blinders_to_poly(rng, 2, &mut poly_2);
        for (ele, expect) in domain.elements().zip(evals.iter()) {
            let res = poly_2.evaluate(&ele);
            assert_eq!(&res, expect);
        }
        // add 3 blinders
        let mut poly_3 = poly.clone();
        add_blinders_to_poly(rng, 3, &mut poly_3);
        for (ele, expect) in domain.elements().zip(evals.iter()) {
            let res = poly_3.evaluate(&ele);
            assert_eq!(&res, expect);
        }
    }

    #[test]
    fn test_generate_s_evals() {
        let rng = &mut test_rng();

        let mut b_evals = (0..16).into_iter().map(|_| Fr::rand(rng)).collect_vec();
        let s_evals = generate_s_evals(&b_evals);
        let shifted_s = [&s_evals[1..], &s_evals[..1]].concat();

        let b_sum: Fr = b_evals.iter().sum();
        b_evals[0] -= b_sum;
        itertools::izip!(b_evals, s_evals, shifted_s)
            .for_each(|(b, s, ss)| {
                assert_eq!(ss - s, b)
            });
    }

    #[test]
    fn test_generate_h_evals() {
        let rng = &mut test_rng();

        let b_evals = (0..16).into_iter().map(|_| {
            let b = rng.gen_range(0..10u64);
            Fr::from(b)
        }).collect_vec();

        let (h1_evals, h2_evals) = generate_h_evals(&b_evals);
        // Check that h1_evals are in increasing order
        h1_evals
            .iter()
            .zip(h1_evals.iter().skip(1))
            .for_each(|(&h1, &h1_next)| {
                let diff = h1_next - h1;
                assert!(diff == Fr::one() || diff == Fr::zero());
            });
        // Check that h2_evals are in increasing order
        h2_evals
            .iter()
            .zip(h2_evals.iter().skip(1))
            .for_each(|(&h2, &h2_next)| {
                let diff = h2_next - h2;
                assert!(diff == Fr::one() || diff == Fr::zero());
            });
        assert_eq!(h1_evals.first().unwrap(), &Fr::zero());
        assert_eq!(h2_evals.last().unwrap(), &Fr::from(15u64));
    }

    #[test]
    fn test_generate_z_evals() {
        let rng = &mut test_rng();

        let b_evals = (0..16).into_iter().map(|_| {
            let b = rng.gen_range(0..8u64);
            Fr::from(b)
        }).collect_vec();

        let (h1_evals, h2_evals) = generate_h_evals(&b_evals);
        let gamma = Fr::rand(rng);
        let z_evals = generate_z_evals(gamma, &b_evals, &h1_evals, &h2_evals);
        let shifted_z = [&z_evals[1..], &z_evals[..1]].concat();
        itertools::izip!(b_evals, z_evals, shifted_z, h1_evals, h2_evals)
            .enumerate()
            .for_each(|(i, (b, z, sz, h1, h2))| {
                let left = z * (gamma + b) * (gamma + Fr::from(i as u64));
                let right = sz * (gamma + h1) * (gamma + h2);
                assert_eq!(left, right);
            });
    }

    #[test]
    fn test_full() {
        let rng = &mut test_rng();

        let n = 16;
        // setup
        let max_degree = if cfg!(blinding) { n + 3 } else { n };
        let pp = KZG10::<Bn254>::setup(max_degree, None, rng).unwrap();
        let (ck, cvk) = KZG10::<Bn254>::trim(
            &pp,
            max_degree,
            0,
            None,
        ).unwrap();

        // precompute
        let (labeled_t_poly, labeled_t_commit) =
            precompute::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(&ck, n).unwrap();

        // generate random balances
        let balances = (0..n)
            .into_iter()
            .map(|_| rng.gen_range(0..8u64))
            .collect_vec();

        // Proof of Balance Sum
        // prove
        let (m, proof, labeled_b_poly) =
            prove::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>, MerlinTranscript, _>(
                &ck,
                n,
                &labeled_t_poly,
                &labeled_t_commit,
                &balances,
                rng,
            ).unwrap();
        // verify
        let res = proof.verify::<MerlinTranscript>(&cvk, n, &labeled_t_commit, m);
        assert!(res.is_ok());

        // Individual Checking
        let i = rng.gen_range(0..n);
        let balance = balances[i];
        // open
        let opening_proof = individual_open::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(
            &ck,
            n,
            i,
            &labeled_b_poly,
            &proof.b_commit,
        ).unwrap();
        // verify
        let res = individual_verify::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(
            &cvk,
            n,
            i,
            balance,
            &proof.b_commit,
            &opening_proof,
        );
        assert!(res.is_ok());
    }
}
