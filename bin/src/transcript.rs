use ark_bn254::{Fr, Bn254};
use ark_ff::{BigInteger, PrimeField, FromBytes};
use ethereum_types::H256;
use sha3::{Digest, Keccak256};
use posol_core::{commitment::KZG10Commitment, balance_sum::TranscriptProtocol};

const DST_0: u8 = 0;
const DST_1: u8 = 1;
const DST_CHALLENGE: u8 = 2;

pub struct Transcript {
    state_0: H256,
    state_1: H256,
    counter: u32,
}

impl Transcript {
    fn append_bytes_without_label(&mut self, item: &[u8]) {
        let old_state = self.state_0.clone();

        let mut data = Vec::with_capacity(1 + 32 + 32 + item.len());
        data.push(DST_0);
        data.extend_from_slice(&old_state[..]);
        data.extend_from_slice(self.state_1.as_bytes());
        data.extend_from_slice(item);
        self.state_0.as_mut().copy_from_slice(&Keccak256::digest(&data)[..32]);

        data.clear();
        data.push(DST_1);
        data.extend_from_slice(&old_state[..]);
        data.extend_from_slice(self.state_1.as_bytes());
        data.extend_from_slice(item);
        self.state_1.as_mut().copy_from_slice(&Keccak256::digest(&data)[..32]);
    }
}

impl TranscriptProtocol<Fr, KZG10Commitment<Bn254>> for Transcript {
    fn new(_label: &'static str) -> Self {
        Self {
            state_0: H256::zero(),
            state_1: H256::zero(),
            counter: 0,
        }
    }

    fn append_u64(&mut self, _label: &'static str, item: u64) {
        self.append_bytes_without_label(&item.to_be_bytes());
    }

    fn append_scalar(&mut self, _label: &'static str, item: &Fr) {
        self.append_bytes_without_label(&item.into_repr().to_bytes_be());
    }

    fn append_commitment(&mut self, _label: &'static str, item: &KZG10Commitment<Bn254>) {
        self.append_bytes_without_label(&item.0.x.into_repr().to_bytes_be());
        self.append_bytes_without_label(&item.0.y.into_repr().to_bytes_be());
    }

    fn challenge_scalar(&mut self, _label: &'static str) -> Fr {
        let mut data = Vec::with_capacity(1 + 32 + 32 + 4);
        data.push(DST_CHALLENGE);
        data.extend_from_slice(self.state_0.as_bytes());
        data.extend_from_slice(self.state_1.as_bytes());
        data.extend(self.counter.to_be_bytes());
        self.counter += 1;
        
        let mut query = Keccak256::digest(&data);
        query.reverse();
        query[31] &= 0x1f;
        Fr::read(&query[..32]).unwrap()
    }
}

#[cfg(test)]
mod test {
    use ark_bn254::{G1Affine, Fq};
    use ark_poly_commit::kzg10::Commitment;
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_transcript() {
        let mut transcript = Transcript::new("test");
        transcript.append_u64("a", 1);
        let a = transcript.challenge_scalar("a");
        assert_eq!(
            a.into_repr().to_bytes_be(),
            hex!("0f9d11cec4f06b0d18060cde3db4196495ddfbb096108951446fc8a1d45f4b59"),
        );

        transcript.append_scalar("b", &Fr::from(2));
        let b = transcript.challenge_scalar("b");
        assert_eq!(
            b.into_repr().to_bytes_be(),
            hex!("0f4dccb919a5dba2dd010a562ba45b4551291f5e565706536e78b24ac8b5c64d"),
        );

        transcript.append_commitment(
            "c",
            &Commitment::<Bn254>(G1Affine::new(Fq::from(3), Fq::from(4), false)),
        );
        let c = transcript.challenge_scalar("c");
        assert_eq!(
            c.into_repr().to_bytes_be(),
            hex!("1b5bf46adfcd1dd4f9ac7166586cf83f261192bc4b83fdda30ddee22f9054c1f"),
        );
    }
}
