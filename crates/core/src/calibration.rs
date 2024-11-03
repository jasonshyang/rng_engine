use crate::{
    calibration_strategy::{
        CalibrationContext,
        StrategyManager,
    }, 
    config::CalibrationConfig, 
    db::RngRecordEntry
};

pub struct Calibration {
    context: CalibrationContext,
}

impl Calibration {
    pub fn new() -> Self {
        Calibration {
            context: CalibrationContext::new(),
        }
    }

    pub fn calibrate(&mut self, base_rng_result: i64, history: Vec<RngRecordEntry>, config: &CalibrationConfig) -> i64 {
        // load history
        let mut history_results = Vec::new();
        for entry in history {
            history_results.push(entry.roll_result);
        }
        StrategyManager::update_strategies(&mut self.context, &history_results); // will need to create config for this

        self.context.calibrate(base_rng_result, &history_results, config)
    }
}