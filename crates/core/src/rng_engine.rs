use crate::{
    base_rng::BaseRng,
    calibration::Calibration,
    db::Db,
    config::CalibrationConfig,
};
use std::sync::Arc;

// RngEngine is the main struct that generates random numbers
// It uses BaseRng to generate a random number, then calibrates the result using Calibration
// It is also responsible for interacting with db to read and write RNG results

pub struct RngEngine {
    base_rng: BaseRng,
    calibration: Calibration,
    db: Arc<Db>,
    calibration_config: CalibrationConfig,
}

impl RngEngine {
    pub fn new(seed: u64, db: Arc<Db>, calibration_config: CalibrationConfig) -> Self {
        RngEngine {
            base_rng: BaseRng::new(seed),
            calibration: Calibration::new(),
            db,
            calibration_config,
        }
    }

    pub async fn generate(&mut self, player_id: i64) -> Result<i64, Box<dyn std::error::Error + Send + Sync>> {
        let base_rng_result = self.base_rng.generate();

        let history = match self.db.get_rolls(player_id, 10).await? {
            Some(history) => history,
            None => Vec::new(),
        };

        let calibrated_result = self.calibration.calibrate(base_rng_result, history, &self.calibration_config);
        
        self.db.insert_roll(player_id, calibrated_result).await?;

        Ok(calibrated_result)
    }
}

