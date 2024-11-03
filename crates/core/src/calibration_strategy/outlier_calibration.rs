use super::CalibrationStrategy;
use crate::config::CalibrationConfig;

pub struct OutlierCalibration;

impl CalibrationStrategy for OutlierCalibration {
    fn calibrate(&self, base_rng_result: i64, history: &[i64], config: &CalibrationConfig) -> i64 {
        print!("Running OutlierCalibration strategy\n");

        // placeholder logics for now
        if history.is_empty() {
            return base_rng_result;
        }
        
        print!("Result changed from {} by multiplying {}\n", base_rng_result, config.outlier_adj_factor);
        (base_rng_result as f64 * config.outlier_adj_factor) as i64
    }
}