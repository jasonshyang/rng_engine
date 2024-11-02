use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub struct BaseRng {
    rng: StdRng,
}

impl BaseRng {
    pub fn new(seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);
        BaseRng { rng }
    }

    pub fn generate(&mut self) -> i64 {
        self.rng.gen_range(0..100)
    }

    pub fn generate_range(&mut self, min: i64, max: i64) -> i64 {
        self.rng.gen_range(min..max)
    }
}