use super::CalibrationStrategy;
use crate::config::CalibrationConfig;
pub struct MedCalibration;

impl CalibrationStrategy for MedCalibration {
    fn calibrate(&self, base_rng_result: i64, history: &[i64], config: &CalibrationConfig) -> i64 {
        print!("Running MedCalibration strategy\n");

        // placeholder logics for now
        if history.is_empty() {
            return base_rng_result;
        }
        let mut sorted_history = history.to_vec();
        sorted_history.sort();
        let median = sorted_history[sorted_history.len() / 2];
        let diff = median - config.target_mean;
        let mut result = base_rng_result - diff;
        if result < config.min {
            result = config.min;
        } else if result > config.max {
            result = config.max;
        }

        print!("Result changed from {} to {}\n", base_rng_result, result);
        result

    }
}