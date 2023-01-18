//! Useful commitment stuff
use ark_ec::{msm::VariableBaseMSM, AffineCurve, PairingEngine};
use ark_ff::{Field, PrimeField};
use ark_poly::univariate::DensePolynomial;
use ark_poly_commit::{sonic_pc::SonicKZG10, ipa_pc::InnerProductArgPC, PolynomialCommitment};

/// A homomorphic polynomial commitment
pub trait HomomorphicCommitment<F>:
    PolynomialCommitment<F, DensePolynomial<F>> + 'static
where
    F: Field,
    Self::VerifierKey: core::fmt::Debug,
{
    /// Combine a linear combination of homomorphic commitments
    fn multi_scalar_mul(
        commitments: &[Self::Commitment],
        scalars: &[F],
    ) -> Self::Commitment;
}

/// The Default KZG-style commitment scheme
pub type KZG10<E> = SonicKZG10<E, DensePolynomial<<E as PairingEngine>::Fr>>;
/// A single KZG10 commitment
pub type KZG10Commitment<E> = <KZG10<E> as PolynomialCommitment<
    <E as PairingEngine>::Fr,
    DensePolynomial<<E as PairingEngine>::Fr>,
>>::Commitment;

impl<E> HomomorphicCommitment<E::Fr> for KZG10<E>
where
    E: PairingEngine,
{
    fn multi_scalar_mul(
        commitments: &[KZG10Commitment<E>],
        scalars: &[E::Fr],
    ) -> KZG10Commitment<E> {
        let scalars_repr = scalars
            .iter()
            .map(<E::Fr as PrimeField>::into_repr)
            .collect_vec();

        let points_repr = commitments.iter().map(|c| c.0).collect_vec();
        let commitment = VariableBaseMSM::multi_scalar_mul(&points_repr, &scalars_repr).into();
        
        ark_poly_commit::kzg10::Commitment::<E>(commitment)
    }
}

/// Shortened type for Inner Product Argument polynomial commitment schemes
pub type IPA<G, D> = InnerProductArgPC<
    G,
    D,
    DensePolynomial<<G as ark_ec::AffineCurve>::ScalarField>,
>;
/// Shortened type for an Inner Product Argument polynomial commitment
pub type IPACommitment<G, D> = <IPA<G, D> as PolynomialCommitment<
    <G as AffineCurve>::ScalarField,
    DensePolynomial<<G as AffineCurve>::ScalarField>,
>>::Commitment;

use blake2::digest::{Digest, Update};
use itertools::Itertools;
impl<G, D> HomomorphicCommitment<<G as ark_ec::AffineCurve>::ScalarField>
    for IPA<G, D>
where
    G: AffineCurve,
    D: Digest + Update + 'static,
{
    fn multi_scalar_mul(
        commitments: &[IPACommitment<G, D>],
        scalars: &[<G as ark_ec::AffineCurve>::ScalarField],
    ) -> IPACommitment<G, D> {
        let scalars_repr = scalars
            .iter()
            .map(<G as ark_ec::AffineCurve>::ScalarField::into_repr)
            .collect_vec();

        let points_repr = commitments.iter().map(|c| c.comm).collect_vec();

        IPACommitment::<G, D> {
            comm: VariableBaseMSM::multi_scalar_mul(
                &points_repr,
                &scalars_repr,
            )
            .into(),
            shifted_comm: None, // TODO: support degree bounds?
        }
    }
}
