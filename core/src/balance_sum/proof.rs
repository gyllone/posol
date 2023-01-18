use std::marker::PhantomData;
use ark_std::{start_timer, end_timer};
use ark_ff::FftField;
use ark_poly::EvaluationDomain;
use ark_serialize::*;
use anyhow::{anyhow, Result};

use crate::{
    util::{EvaluationDomainExt, compute_lagrange_evaluation},
    commitment::HomomorphicCommitment,
    label_commitment,
};
use super::transcript::TranscriptProtocol;

#[derive(Debug, Clone, Eq, PartialEq, CanonicalDeserialize, CanonicalSerialize)]
pub struct Evaluations<F: FftField> {
    pub b: F,
    pub t: F,
    pub h1: F,
    pub h2: F,

    pub s_next: F,
    pub z_next: F,
    pub h1_next: F,
    pub h2_next: F,
}

#[derive(CanonicalDeserialize, CanonicalSerialize, derivative::Derivative)]
#[derivative(
    Clone(bound = "PC::Commitment: Clone, PC::Proof: Clone"),
    Debug(bound = "PC::Commitment: core::fmt::Debug, PC::Proof: core::fmt::Debug"),
    Eq(bound = "PC::Commitment: Eq, PC::Proof: Eq"),
    PartialEq(bound = "PC::Commitment: PartialEq, PC::Proof: PartialEq")
)]
pub struct Proof<F, D, PC>
where
    F: FftField,
    D: EvaluationDomain<F> + EvaluationDomainExt<F>,
    PC: HomomorphicCommitment<F>,
{
    pub b_commit: PC::Commitment,
    pub s_commit: PC::Commitment,
    pub h1_commit: PC::Commitment,
    pub h2_commit: PC::Commitment,
    pub z_commit: PC::Commitment,
    pub q1_commit: PC::Commitment,
    pub q2_commit: PC::Commitment,

    pub w_opening: PC::Proof,
    pub sw_opening: PC::Proof,

    pub evaluations: Evaluations<F>,

    pub(super) _p: PhantomData<D>,
}

impl<F, D, PC> Proof<F, D, PC>
where
    F: FftField,
    D: EvaluationDomain<F> + EvaluationDomainExt<F>,
    PC: HomomorphicCommitment<F>,
{
    fn compute_r_eval(
        &self,
        n: u64,
        gamma: F,
        deltas: &[F],
        l0_eval: F,
        ln_eval: F,
        m: F,
    ) -> F {
        - self.evaluations.s_next - m * l0_eval
        + self.evaluations.z_next * (gamma + self.evaluations.h1) * gamma * deltas[0]
        + l0_eval * deltas[1]
        - self.evaluations.h1_next
            * (self.evaluations.h1_next - self.evaluations.h1 - F::one())
            * (ln_eval - F::one())
            * deltas[2]
        - self.evaluations.h2_next
            * (self.evaluations.h2_next - self.evaluations.h2 - F::one())
            * (ln_eval - F::one())
            * deltas[3]
        - self.evaluations.h2_next
            * (self.evaluations.h2_next - self.evaluations.h1 - F::one())
            * ln_eval
            * deltas[4]
        + F::from(n - 1) * ln_eval * deltas[6]
    }

    fn linearisation_commitments(
        &self,
        gamma: F,
        deltas: &[F],
        z: F,
        zh_eval: F,
        l0_eval: F,
        ln_eval: F,
    ) -> PC::Commitment {
        let mut scalars = Vec::with_capacity(7);
        let mut commitments = Vec::with_capacity(7);

        scalars.push(-F::one());
        commitments.push(self.s_commit.clone());
        
        scalars.push(-F::one());
        commitments.push(self.b_commit.clone());
        
        scalars.push(
            (gamma + self.evaluations.b) * (gamma + self.evaluations.t) * deltas[0] + l0_eval * deltas[1]
        );
        commitments.push(self.z_commit.clone());

        scalars.push(
            l0_eval * deltas[5]
            - (self.evaluations.h1_next - self.evaluations.h1 - F::one()) * (ln_eval - F::one()) * deltas[2]
            - (self.evaluations.h2_next - self.evaluations.h1 - F::one()) * ln_eval * deltas[4]
        );
        commitments.push(self.h1_commit.clone());

        scalars.push(
            ln_eval * deltas[6]
            - self.evaluations.z_next * (gamma + self.evaluations.h1)  * deltas[0]
            - (self.evaluations.h2_next - self.evaluations.h2 - F::one()) * (ln_eval - F::one()) * deltas[3]
        );
        commitments.push(self.h2_commit.clone());

        scalars.push(-zh_eval);
        commitments.push(self.q1_commit.clone());

        scalars.push(-zh_eval * (zh_eval + F::one()) * z.square() * z);
        commitments.push(self.q2_commit.clone());

        PC::multi_scalar_mul(&commitments, &scalars)
    }

    pub fn verify<T>(
        &self,
        cvk: &PC::VerifierKey,
        n: usize,
        t_commit: &PC::Commitment,
        m: F,
    ) -> Result<()>
    where
        T: TranscriptProtocol<F, PC::Commitment>,
    {
        assert!(n.is_power_of_two());

        let domain = D::new(n)
            .ok_or(anyhow!(
                "log size of group: {}, 2-adicity: {}",
                n.trailing_zeros(),
                <F::FftParams as ark_ff::FftParameters>::TWO_ADICITY,
            ))?;
        assert_eq!(n, domain.size());

        let timer = start_timer!(|| "Balance Sum: Verifying");

        let transcript = &mut T::new("Proof of Balance Sum");
        transcript.append_u64("n", n as u64);

        // Append m to the transcript.
        transcript.append_scalar("m", &m);

        transcript.append_commitment("b_commit", &self.b_commit);
        transcript.append_commitment("s_commit", &self.s_commit);
        transcript.append_commitment("h1_commit", &self.h1_commit);
        transcript.append_commitment("h2_commit", &self.h2_commit);

        let gamma = transcript.challenge_scalar("gamma");

        transcript.append_commitment("z_commit", &self.z_commit);

        let delta = transcript.challenge_scalar("delta");

        transcript.append_commitment("q1_commit", &self.q1_commit);
        transcript.append_commitment("q2_commit", &self.q2_commit);

        // Compute evaluation point challenge `z`.
        let z = transcript.challenge_scalar("z");

        // Compute zero polynomial evaluated at `z`
        let zh_eval = domain.evaluate_vanishing_polynomial(z);
        let l0_eval = compute_lagrange_evaluation(n, domain.element(0), zh_eval, z);
        let ln_eval = compute_lagrange_evaluation(n, domain.element(n - 1), zh_eval, z);

        let delta_exp_2 = delta.square();
        let delta_exp_3 = delta_exp_2 * delta;
        let delta_exp_4 = delta_exp_3 * delta;
        let delta_exp_5 = delta_exp_4 * delta;
        let delta_exp_6 = delta_exp_5 * delta;
        let delta_exp_7 = delta_exp_6 * delta;
        let deltas = [delta, delta_exp_2, delta_exp_3, delta_exp_4, delta_exp_5, delta_exp_6, delta_exp_7];

        let r_eval = self.compute_r_eval(
            n as u64,
            gamma,
            &deltas,
            l0_eval,
            ln_eval,
            m,
        );

        let r_commit = self.linearisation_commitments(
            gamma,
            &deltas,
            z,
            zh_eval,
            l0_eval,
            ln_eval,
        );

        transcript.append_scalar("t_eval", &self.evaluations.t);
        transcript.append_scalar("b_eval", &self.evaluations.b);
        transcript.append_scalar("h1_eval", &self.evaluations.h1);
        transcript.append_scalar("h2_eval", &self.evaluations.h2);
        transcript.append_scalar("s_next_eval", &self.evaluations.s_next);
        transcript.append_scalar("h1_next_eval", &self.evaluations.h1_next);
        transcript.append_scalar("h2_next_eval", &self.evaluations.h2_next);
        transcript.append_scalar("z_next_eval", &self.evaluations.z_next);

        // Compute opening point challenge `eta`.
        let eta = transcript.challenge_scalar("eta");

        let labeled_r_commit = label_commitment!(r_commit);
        let labeled_t_commit = label_commitment!(t_commit);
        let labeled_b_commit = label_commitment!(self.b_commit);
        let labeled_h1_commit = label_commitment!(self.h1_commit);
        let labeled_h2_commit = label_commitment!(self.h2_commit);

        match PC::check(
            cvk,
            vec![
                &labeled_r_commit,
                &labeled_t_commit,
                &labeled_b_commit,
                &labeled_h1_commit,
                &labeled_h2_commit,
            ],
            &z,
            vec![
                r_eval,
                self.evaluations.t,
                self.evaluations.b,
                self.evaluations.h1,
                self.evaluations.h2,
            ],
            &self.w_opening,
            eta,
            None,
        ) {
            Ok(true) => Ok(()),
            Ok(false) => Err(anyhow!("verification of w opening failed")),
            Err(e) => Err(anyhow!("check opening W(X) error: {}", e)),
        }
        .and_then(|_| {
            let labeled_s_commit = label_commitment!(self.s_commit);
            let labeled_z_commit = label_commitment!(self.z_commit);

            match PC::check(
                cvk,
                vec![
                    &labeled_s_commit,
                    &labeled_h1_commit,
                    &labeled_h2_commit,
                    &labeled_z_commit,
                ],
                &(z * domain.group_gen()),
                vec![
                    self.evaluations.s_next,
                    self.evaluations.h1_next,
                    self.evaluations.h2_next,
                    self.evaluations.z_next,
                ],
                &self.sw_opening,
                eta,
                None,
            ) {
                Ok(true) => {
                    end_timer!(timer);
                    Ok(())
                }
                Ok(false) => Err(anyhow!("verification of sw opening failed")),
                Err(e) => Err(anyhow!("check opening W_next(X) error: {}", e)),
            }
        })
    }
}
