use super::CalibrationStrategy;
use crate::config::CalibrationConfig;
pub struct AvgCalibration;

impl CalibrationStrategy for AvgCalibration {
    fn calibrate(&self, base_rng_result: i64, history: &[i64], config: &CalibrationConfig) -> i64 {
        print!("Running AvgCalibration strategy\n");
        
        // placeholder logics for now
        if history.is_empty() {
            return base_rng_result;
        }
        let sum: i64 = history.iter().sum();
        let avg = sum / history.len() as i64;
        let diff = avg - config.target_mean;
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