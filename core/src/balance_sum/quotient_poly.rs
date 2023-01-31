use ark_ff::FftField;
use ark_std::{start_timer, end_timer};
use ark_poly::{univariate::DensePolynomial, EvaluationDomain, Polynomial};
use anyhow::{anyhow, Result};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::util::{
    coset_evals_from_poly,
    coset_evals_from_poly_ref,
    poly_from_coset_evals,
    poly_from_evals,
};

#[allow(clippy::too_many_arguments)]
pub(crate) fn compute<F, D>(
    domain: &D,
    m: F,
    gamma: F,
    delta: F,
    t_poly: &DensePolynomial<F>,
    b_poly: &DensePolynomial<F>,
    s_poly: &DensePolynomial<F>,
    h1_poly: &DensePolynomial<F>,
    h2_poly: &DensePolynomial<F>,
    z_poly: &DensePolynomial<F>,
) -> Result<DensePolynomial<F>>
where
    F: FftField,
    D: EvaluationDomain<F>,
{
    let timer = start_timer!(|| "Balance Sum: Computing Quotient Polynomial");

    let n = domain.size();
    if cfg!(blinding) {
        // Size of quotient poly is 2n+6 <= 4n => n >= 3
        assert!(n >= 3);
    }

    let multiplier = if cfg!(blinding) { 4 } else { 2 };
    let extended_domain = D::new(multiplier * n)
        .ok_or(anyhow!(
            "log size of group: {}, 2-adicity: {}",
            (multiplier * n).trailing_zeros(),
            <F::FftParams as ark_ff::FftParameters>::TWO_ADICITY,
        ))?;

    let t_coset = coset_evals_from_poly_ref(&extended_domain, t_poly);
    let b_coset = coset_evals_from_poly_ref(&extended_domain, b_poly);

    let mut s_coset = coset_evals_from_poly_ref(&extended_domain, s_poly);
    s_coset.push(s_coset[0]);
    s_coset.push(s_coset[1]);
    if cfg!(blinding) {
        s_coset.push(s_coset[2]);
        s_coset.push(s_coset[3]);
    }

    let mut h1_coset = coset_evals_from_poly_ref(&extended_domain, h1_poly);
    h1_coset.push(h1_coset[0]);
    h1_coset.push(h1_coset[1]);
    if cfg!(blinding) {
        h1_coset.push(h1_coset[2]);
        h1_coset.push(h1_coset[3]);
    }

    let mut h2_coset = coset_evals_from_poly_ref(&extended_domain, h2_poly);
    h2_coset.push(h2_coset[0]);
    h2_coset.push(h2_coset[1]);
    if cfg!(blinding) {
        h2_coset.push(h2_coset[2]);
        h2_coset.push(h2_coset[3]);
    }

    let mut z_coset = coset_evals_from_poly_ref(&extended_domain, z_poly);
    z_coset.push(z_coset[0]);
    z_coset.push(z_coset[1]);
    if cfg!(blinding) {
        z_coset.push(z_coset[2]);
        z_coset.push(z_coset[3]);
    }

    // Compute extended evaluations for x^n - 1
    let vh_poly: DensePolynomial<_> = domain.vanishing_polynomial().into();
    let vh_coset = coset_evals_from_poly(&extended_domain, vh_poly);

    // compute extended evaluations for L0(x)
    let mut l0_evals = vec![F::zero(); n];
    l0_evals[0] = F::one();
    let l0_poly = poly_from_evals(domain, l0_evals);
    let l0_coset = coset_evals_from_poly(&extended_domain, l0_poly);

    // compute extended evaluations for L{n-1}(x)
    let mut ln_evals = vec![F::zero(); n];
    ln_evals[n - 1] = F::one();
    let ln_poly = poly_from_evals(domain, ln_evals);
    let ln_coset = coset_evals_from_poly(&extended_domain, ln_poly);

    #[cfg(not(feature = "parallel"))]
    let quotient_iter = itertools::izip!(
        t_coset,
        b_coset,
        s_coset.iter(),
        s_coset.iter().skip(multiplier),
        h1_coset.iter(),
        h1_coset.iter().skip(multiplier),
        h2_coset.iter(),
        h2_coset.iter().skip(multiplier),
        z_coset.iter(),
        z_coset.iter().skip(multiplier),
        vh_coset,
        l0_coset,
        ln_coset,
    );
    #[cfg(feature = "parallel")]
    let quotient_iter = crate::par_izip!(
        t_coset,
        b_coset,
        s_coset.par_iter(),
        s_coset.par_iter().skip(multiplier),
        h1_coset.par_iter(),
        h1_coset.par_iter().skip(multiplier),
        h2_coset.par_iter(),
        h2_coset.par_iter().skip(multiplier),
        z_coset.par_iter(),
        z_coset.par_iter().skip(multiplier),
        vh_coset,
        l0_coset,
        ln_coset,
    );

    let delta_exp_2 = delta.square();
    let delta_exp_3 = delta_exp_2 * delta;
    let delta_exp_4 = delta_exp_3 * delta;
    let delta_exp_5 = delta_exp_4 * delta;
    let delta_exp_6 = delta_exp_5 * delta;
    let delta_exp_7 = delta_exp_6 * delta;
    let q_evals = quotient_iter
        .map(|(t, b, &s, &s_next, &h1, &h1_next, &h2, &h2_next, &z, &z_next, vh, l0, ln)| {
            let q_eval = s_next - s + m * l0 - b
                + z * (gamma + b) * (gamma + t) * delta
                - z_next * (gamma + h1) * (gamma + h2) * delta
                + (z - F::one()) * l0 * delta_exp_2
                + (h1_next - h1) * (h1_next - h1 - F::one()) * (ln - F::one()) * delta_exp_3
                + (h2_next - h2) * (h2_next - h2 - F::one()) * (ln - F::one()) * delta_exp_4
                + (h2_next - h1) * (h2_next - h1 - F::one()) * ln * delta_exp_5
                + h1 * l0 * delta_exp_6
                + (h2 - F::from(n as u64 - 1)) * ln * delta_exp_7;

            q_eval * vh.inverse().unwrap()
        })
        .collect();

    let q_poly = poly_from_coset_evals(&extended_domain, q_evals);
    // Sanity check
    if cfg!(blinding) {
        assert!(q_poly.degree() <= 2 * n + 6);
    } else {
        assert!(q_poly.degree() <= 2 * n);
    }

    end_timer!(timer);

    Ok(q_poly)
}
