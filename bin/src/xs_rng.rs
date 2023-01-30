use rand_core::{CryptoRng, RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

pub struct XSRng(XorShiftRng);

impl RngCore for XSRng {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.0.try_fill_bytes(dest)
    }
}

impl CryptoRng for XSRng {}

pub(crate) fn get_xorshift_rng() -> XSRng {
    XSRng(XorShiftRng::from_entropy())
}
