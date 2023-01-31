use ark_ff::{FftField, Field};
use ark_poly::{
    EvaluationDomain,
    GeneralEvaluationDomain,
    univariate::DensePolynomial,
    UVPolynomial,
};

/// Lagrange polynomial has the expression:
///
/// ```text
/// L_k(X) = ∏ 0 to (n-1) without k [(x - omega^i) / (omega^k - omega^i)]
/// ```
///
/// with `omega` being the generator of the domain (the `n`th root of unity).
///
/// We use two equalities:
///   1. `∏ 0 to (n-1) without k (omega^k - omega^i) = n / omega^k` NOTE: L'Hôpital Principle
///   2. `∏ 0 to (n-1) without k (x - omega^i) = (x^n - 1) / (x - omega^k)`
/// to obtain the expression:
///
/// ```text
/// L_k(X) = (x^n - 1) * omega^k / n * (x - omega^k)
/// ```
pub(crate) fn compute_lagrange_evaluation<F: Field>(
    n: usize,
    point: F,
    zh_eval: F,
    z: F,
) -> F {
    let numinator = zh_eval * point;
    let dominator = F::from(n as u64) * (z - point);
    numinator * dominator.inverse().unwrap()
}

/// Macro to quickly label polynomials
#[macro_export]
macro_rules! label_polynomial {
    ($poly:expr) => {
        ark_poly_commit::LabeledPolynomial::new(
            stringify!($poly).to_owned(),
            $poly,
            None,
            None,
        )
    };
}

/// Macro to quickly label polynomial commitments
#[macro_export]
macro_rules! label_commitment {
    ($comm:expr) => {
        ark_poly_commit::LabeledCommitment::new(
            stringify!($comm).to_owned(),
            $comm.clone(),
            None,
        )
    };
}

/// Evaluation Domain Extension Trait
pub trait EvaluationDomainExt<F>: EvaluationDomain<F>
where
    F: FftField,
{
    /// Returns a fixed generator of the subgroup.
    fn group_gen(&self) -> F;
    /// Returns the inverse of the fixed generator of the subgroup.
    fn group_gen_inv(&self) -> F;
}

impl<F> EvaluationDomainExt<F> for GeneralEvaluationDomain<F>
where
    F: FftField,
{
    #[inline]
    fn group_gen(&self) -> F {
        match self {
            GeneralEvaluationDomain::Radix2(domain) => domain.group_gen,
            GeneralEvaluationDomain::MixedRadix(domain) => domain.group_gen,
        }
    }

    #[inline]
    fn group_gen_inv(&self) -> F {
        match self {
            GeneralEvaluationDomain::Radix2(domain) => domain.group_gen_inv,
            GeneralEvaluationDomain::MixedRadix(domain) => domain.group_gen_inv,
        }
    }
}

///
#[inline]
pub(crate) fn poly_from_evals_ref<F, D>(
    domain: &D,
    evals: &[F],
) -> DensePolynomial<F>
where
    F: FftField,
    D: EvaluationDomain<F>,
{
    DensePolynomial::from_coefficients_vec(domain.ifft(evals))
}

///
#[inline]
pub(crate) fn poly_from_evals<F, D>(
    domain: &D,
    mut evals: Vec<F>,
) -> DensePolynomial<F>
where
    F: FftField,
    D: EvaluationDomain<F>,
{
    domain.ifft_in_place(&mut evals);
    DensePolynomial::from_coefficients_vec(evals)
}

///
#[inline]
pub(crate) fn poly_from_coset_evals<F, D>(
    domain: &D,
    mut evals: Vec<F>,
) -> DensePolynomial<F>
where
    F: FftField,
    D: EvaluationDomain<F>,
{
    domain.coset_ifft_in_place(&mut evals);
    DensePolynomial::from_coefficients_vec(evals)
}

///
#[inline]
pub(crate) fn coset_evals_from_poly<F, D>(
    domain: &D,
    mut poly: DensePolynomial<F>,
) -> Vec<F>
where
    F: FftField,
    D: EvaluationDomain<F>,
{
    domain.coset_fft_in_place(&mut poly.coeffs);
    poly.coeffs
}

///
#[inline]
pub(crate) fn coset_evals_from_poly_ref<F, D>(
    domain: &D,
    poly: &DensePolynomial<F>,
) -> Vec<F>
where
    F: FftField,
    D: EvaluationDomain<F>,
{
    domain.coset_fft(poly)
}

///
#[cfg(feature = "parallel")]
#[macro_export]
macro_rules! par_izip {
    // @closure creates a tuple-flattening closure for .map() call. usage:
    // @closure partial_pattern => partial_tuple , rest , of , iterators
    // eg. izip!( @closure ((a, b), c) => (a, b, c) , dd , ee )
    ( @closure $p:pat => $tup:expr ) => {
        |$p| $tup
    };

    // The "b" identifier is a different identifier on each recursion level thanks to hygiene.
    ( @closure $p:pat => ( $($tup:tt)* ) , $_iter:expr $( , $tail:expr )* ) => {
        $crate::par_izip!(@closure ($p, b) => ( $($tup)*, b ) $( , $tail )*)
    };

    // unary
    ($first:expr $(,)*) => {
        rayon::iter::IntoParallelIterator::into_par_iter($first)
    };

    // binary
    ($first:expr, $second:expr $(,)*) => {
        $crate::par_izip!($first)
            .zip($second)
    };

    // n-ary where n > 2
    ( $first:expr $( , $rest:expr )* $(,)* ) => {
        $crate::par_izip!($first)
            $(
                .zip($rest)
            )*
            .map(
                $crate::par_izip!(@closure a => (a) $( , $rest )*)
            )
    };
}
