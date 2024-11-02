use crate::{
    base_rng::BaseRng,
    calibration::Calibration,
    db::Db,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RngEngine {
    base_rng: BaseRng,
    calibration: Calibration,
    db: Arc<Mutex<Db>>,
}

impl RngEngine {
    pub fn new(seed: u64, db: Arc<Mutex<Db>>) -> Self {
        RngEngine {
            base_rng: BaseRng::new(seed),
            calibration: Calibration::new(),
            db,
        }
    }

    pub async fn generate(&mut self, player_id: i64) -> Result<i64, Box<dyn std::error::Error + Send + Sync>> {
        let base_rng_result = self.base_rng.generate();
        let calibrated_result = self.calibration.calibrate(player_id, base_rng_result);

        self.db.lock().await.insert_roll(player_id, calibrated_result).await?;

        Ok(calibrated_result)
    }
}

