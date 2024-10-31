use crate::base_rng::BaseRng;
use crate::calibration::Calibration;

pub struct RngEngine {
    base_rng: BaseRng,
    calibration: Calibration,
}

impl RngEngine {
    pub fn new(seed: u64) -> Self {
        RngEngine {
            base_rng: BaseRng::new(seed),
            calibration: Calibration::new(),
        }
    }

    pub fn generate(&mut self, id: u64) -> u32 {
        let base_rng_result = self.base_rng.generate();
        self.calibration.calibrate(id, base_rng_result)
    }
}

