use super::{
    CalibrationContext,
    AvgCalibration,
    MedCalibration,
    OutlierCalibration,
};

// logics to manage which strategies to apply

pub struct StrategyManager;

impl StrategyManager {
    pub fn update_strategies(context: &mut CalibrationContext, history: &[i64]) {
        context.clear_strategies();

        print!("Updating strategies\n");
        // placeholder logics to add strategy based on history
        if history.len() > 5 {
            context.add_strategy(Box::new(AvgCalibration));
        }
        if history.len() > 8 {
            context.add_strategy(Box::new(MedCalibration));
        }
        if Self::is_outlier(history) {
            context.add_strategy(Box::new(OutlierCalibration));
        }
    }

    fn is_outlier(history: &[i64]) -> bool {
        if history.is_empty() {
            return false;
        }
        // to be implemented
        true
    }
}