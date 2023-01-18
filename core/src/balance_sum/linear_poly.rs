use std::ops::{AddAssign, SubAssign};
use ark_ff::FftField;
use ark_poly::{univariate::DensePolynomial, EvaluationDomain, Polynomial};
use num_traits::Zero;

use crate::util::{EvaluationDomainExt, compute_lagrange_evaluation};
use super::proof::Evaluations;

#[allow(clippy::too_many_arguments)]
pub(crate) fn compute<F, D>(
    domain: &D,
    gamma: F,
    delta: F,
    z: F,
    t_poly: &DensePolynomial<F>,
    b_poly: &DensePolynomial<F>,
    s_poly: &DensePolynomial<F>,
    h1_poly: &DensePolynomial<F>,
    h2_poly: &DensePolynomial<F>,
    z_poly: &DensePolynomial<F>,
    q1_poly: &DensePolynomial<F>,
    q2_poly: &DensePolynomial<F>,
) -> (DensePolynomial<F>, Evaluations<F>)
where
    F: FftField,
    D: EvaluationDomain<F> + EvaluationDomainExt<F>,
{
    let n = domain.size();

    let b_eval = b_poly.evaluate(&z);
    let t_eval = t_poly.evaluate(&z);
    let h1_eval = h1_poly.evaluate(&z);
    let h2_eval = h2_poly.evaluate(&z);
    let zh_eval = domain.evaluate_vanishing_polynomial(z);
    let l0_eval = compute_lagrange_evaluation(n, domain.element(0), zh_eval, z);
    let ln_eval = compute_lagrange_evaluation(n, domain.element(n - 1), zh_eval, z);

    let z_next = z * domain.group_gen();
    let s_next_eval = s_poly.evaluate(&z_next);
    let z_next_eval = z_poly.evaluate(&z_next);
    let h1_next_eval = h1_poly.evaluate(&z_next);
    let h2_next_eval = h2_poly.evaluate(&z_next);

    let delta_exp_2 = delta.square();
    let delta_exp_3 = delta_exp_2 * delta;
    let delta_exp_4 = delta_exp_3 * delta;
    let delta_exp_5 = delta_exp_4 * delta;
    let delta_exp_6 = delta_exp_5 * delta;
    let delta_exp_7 = delta_exp_6 * delta;

    let mut r_poly = DensePolynomial::zero();
    r_poly.sub_assign(s_poly);
    r_poly.sub_assign(b_poly);
    r_poly.add_assign(
        &(z_poly * ((gamma + b_eval) * (gamma + t_eval) * delta + l0_eval * delta_exp_2))
    );
    r_poly.add_assign(
        &(h1_poly * (
            l0_eval * delta_exp_6
            - (h1_next_eval - h1_eval - F::one()) * (ln_eval - F::one()) * delta_exp_3
            - (h2_next_eval - h1_eval - F::one()) * ln_eval * delta_exp_5
        ))
    );
    r_poly.add_assign(
        &(h2_poly * (
            ln_eval * delta_exp_7
            - z_next_eval * (gamma + h1_eval) * delta
            - (h2_next_eval - h2_eval - F::one()) * (ln_eval - F::one()) * delta_exp_4
        ))
    );
    r_poly.sub_assign(&(q1_poly * zh_eval));
    r_poly.sub_assign(&(q2_poly * (zh_eval * (zh_eval + F::one()) * z.square() * z)));

    let evaluations = Evaluations {
        b: b_eval,
        t: t_eval,
        h1: h1_eval,
        h2: h2_eval,
        s_next: s_next_eval,
        z_next: z_next_eval,
        h1_next: h1_next_eval,
        h2_next: h2_next_eval,
    };

    (r_poly, evaluations)
}
