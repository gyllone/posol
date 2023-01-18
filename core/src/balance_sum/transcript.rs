
use ark_ff::{Field, PrimeField};
use ark_poly_commit::PCCommitment;
use merlin::Transcript;

/// Transcript adds an abstraction over the Merlin transcript
/// For convenience
pub trait TranscriptProtocol<F, PC>
where
    F: Field,
    PC: PCCommitment + 'static,
{
    ///
    fn new(label: &'static str) -> Self;

    ///
    fn append_u64(&mut self, label: &'static str, item: u64);

    ///
    fn append_scalar(&mut self, label: &'static str, item: &F);

    ///
    fn append_commitment(&mut self, label: &'static str, item: &PC);

    /// Compute a `label`ed challenge variable.
    fn challenge_scalar(&mut self, label: &'static str) -> F;
}

///
#[derive(Clone)]
pub struct MerlinTranscript(Transcript);

impl<F, PC> TranscriptProtocol<F, PC> for MerlinTranscript
where
    F: PrimeField,
    PC: PCCommitment + 'static,
{
    fn new(label: &'static str) -> Self {
        Self(Transcript::new(label.as_bytes()))
    }

    fn append_u64(&mut self, label: &'static str, item: u64) {
        self.0.append_u64(label.as_bytes(), item)
    }

    fn append_scalar(&mut self, label: &'static str, item: &F) {
        let mut bytes = Vec::new();
        item.write(&mut bytes).expect("F can not convert to bytes");

        self.0.append_message(label.as_bytes(), &bytes)
    }

    fn append_commitment(&mut self, label: &'static str, item: &PC) {
        let mut bytes = Vec::new();
        item.write(&mut bytes).expect("PC can not convert to bytes");

        self.0.append_message(label.as_bytes(), &bytes)
    }

    fn challenge_scalar(&mut self, label: &'static str) -> F {
        let num_bytes = (F::size_in_bits() + 7) / 8 - 1;
        let mut bytes = vec![0u8; num_bytes as usize];
        self.0.challenge_bytes(label.as_bytes(), &mut bytes);

        F::from_random_bytes(&bytes).unwrap()
    }
}
