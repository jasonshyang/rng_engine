pub mod avg_calibration;
pub mod med_calibration;
pub mod outlier_calibration;
pub mod strategy_manager;

pub use avg_calibration::AvgCalibration;
pub use med_calibration::MedCalibration;
pub use outlier_calibration::OutlierCalibration;
pub use strategy_manager::StrategyManager;

use crate::config::CalibrationConfig;

pub trait CalibrationStrategy: Send + Sync {
    fn calibrate(&self, base_rng_result: i64, history: &[i64], config: &CalibrationConfig) -> i64;
}

pub struct CalibrationContext {
    strategies: Vec<Box<dyn CalibrationStrategy + Send + Sync>>, // dynamic dispatch is used because the strategy is determined in runtime
}

impl CalibrationContext {
    pub fn new() -> Self {
        CalibrationContext {
            strategies: Vec::new(),
        }
    }

    pub fn add_strategy(&mut self, strategy: Box<dyn CalibrationStrategy + Send + Sync>) {
        self.strategies.push(strategy);
    }

    pub fn clear_strategies(&mut self) {
        self.strategies.clear();
    }

    pub fn calibrate(&self, base_rng_result: i64, history: &[i64], config: &CalibrationConfig) -> i64 {
        let mut result = base_rng_result;
        for strategy in &self.strategies {
            result = strategy.calibrate(result, history, config);
        }

        print!("Final: Result changed from {} to {}\n", base_rng_result, result);
        result
    }
}

// calibration logics
// 1 - Get history of RNG results for player ID
// 2 - Calculate calibration factor based on history: e.g. average, median, etc.
//     - identify outliers, e.g. last x results all below lower bound or above upper bound
//     - identify consistent bias, e.g. last x results all above or below average
//     - consider moving the more complex logics to a separate module
// 3 - If history is empty, use default calibration factor
// 4 - Apply calibration factor to base RNG result
// 5 - Apply any additional logics to the calibrated result
// 6 - Return the calibrated result