use ark_std::{start_timer, end_timer};
use ark_ff::FftField;
use ark_poly::{EvaluationDomain, univariate::DensePolynomial};
use ark_poly_commit::{LabeledPolynomial, PCRandomness};
use anyhow::{anyhow, Result};

use crate::{
    commitment::HomomorphicCommitment,
    util::poly_from_evals,
    label_polynomial, label_commitment,
};

pub fn commit<F, D, PC>(
    ck: &PC::CommitterKey,
    n: usize,
    tags: &[&[u8]],
) -> Result<(PC::Commitment, LabeledPolynomial<F, DensePolynomial<F>>)>
where
    F: FftField,
    D: EvaluationDomain<F>,
    PC: HomomorphicCommitment<F>,
{
    assert!(n.is_power_of_two());
    assert!(tags.len() <= n);

    let domain = D::new(n)
        .ok_or(anyhow!(
            "log size of group: {}, 2-adicity: {}",
            n.trailing_zeros(),
            <F::FftParams as ark_ff::FftParameters>::TWO_ADICITY,
        ))?;

    let timer = start_timer!(|| "Tag: Committing");

    let mut tag_evals = tags
        .iter()
        .map(|&reader| {
            F::read(reader)
                .map_err(|e| anyhow!("failed to read tag: {}", e))
        })
        .collect::<Result<Vec<_>>>()?;
    tag_evals.resize(n, F::zero());
    
    let tag_poly = poly_from_evals(&domain, tag_evals);
    let labeled_tag_poly = label_polynomial!(tag_poly);

    // Commit to tag(X)
    let (labeled_tag_commit, _) =
        PC::commit(ck, vec![&labeled_tag_poly], None)
        .map_err(|e| {
            anyhow!("commit to tag(x) failed: {}", e)
        })?;

    end_timer!(timer);

    Ok((
        labeled_tag_commit[0].commitment().clone(),
        labeled_tag_poly,
    ))
}

pub fn individual_open<F, D, PC>(
    ck: &PC::CommitterKey,
    n: usize,
    i: usize,
    tag_poly: &LabeledPolynomial<F, DensePolynomial<F>>,
    tag_commit: &PC::Commitment,
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

    let timer = start_timer!(|| "Tag: Individual Opening");
    
    let point = domain.element(i);
    let labeled_tag_commit = label_commitment!(tag_commit);
    let randomness = <PC::Randomness as PCRandomness>::empty();
    let proof = PC::open(
        ck,
        vec![tag_poly],
        vec![&labeled_tag_commit],
        &point,
        F::one(),
        vec![&randomness],
        None,
    )
    .map_err(|e| anyhow!("open tag(X) failed: {}", e))?;

    end_timer!(timer);

    Ok(proof)
}

pub fn individual_verify<F, D, PC>(
    vk: &PC::VerifierKey,
    n: usize,
    i: usize,
    tag: &[u8],
    tag_commit: &PC::Commitment,
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

    let timer = start_timer!(|| "Tag: Individual Verifying");

    let point = domain.element(i);
    let evaluation = F::read(tag)
        .map_err(|e| anyhow!("failed to read tag: {}", e))?;
    let labeled_tag_commit = label_commitment!(tag_commit);
    match PC::check(
        vk,
        vec![&labeled_tag_commit],
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
        Ok(false) => Err(anyhow!("individual tag verification failed")),
        Err(e) => Err(anyhow!("check opening proof error: {}", e)),
    }
}

#[cfg(test)]
mod test {
    use ark_ff::{ToBytes, UniformRand};
    use ark_poly::GeneralEvaluationDomain;
    use ark_poly_commit::PolynomialCommitment;
    use ark_std::{test_rng, rand::Rng};
    use ark_bn254::{Fr, Bn254};
    use itertools::Itertools;
    
    use crate::commitment::KZG10;
    use super::*;

    #[test]
    fn test_full() {
        let rng = &mut test_rng();

        let n = 16;
        // setup
        let pp = KZG10::<Bn254>::setup(n + 3, None, rng).unwrap();
        let (ck, cvk) = KZG10::<Bn254>::trim(
            &pp,
            n + 3,
            0,
            None,
        ).unwrap();

        // generate random balances
        let tags = (0..n)
            .into_iter()
            .map(|_| {
                let mut bytes = vec![0u8; 32];
                Fr::rand(rng).write(&mut bytes).unwrap();
                bytes
            })
            .collect_vec();
        let tags_ref = tags.iter().map(|t| &t[..]).collect_vec();

        // commit
        let (tag_commit, labeled_tag_poly) =
            commit::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(&ck, n, &tags_ref).unwrap();

        // Individual Checking
        let i = rng.gen_range(0..n);
        let tag = tags_ref[i];
        // open
        let opening_proof = individual_open::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(
            &ck,
            n,
            i,
            &labeled_tag_poly,
            &tag_commit,
        ).unwrap();
        // verify
        let res = individual_verify::<_, GeneralEvaluationDomain<_>, KZG10<Bn254>>(
            &cvk,
            n,
            i,
            tag,
            &tag_commit,
            &opening_proof,
        );
        assert!(res.is_ok());
    }
}

