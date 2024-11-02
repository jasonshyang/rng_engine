use std::collections::HashMap;

pub struct Calibration {
    history: HashMap<i64, Vec<i64>>,
}

impl Calibration {
    pub fn new() -> Self {
        Calibration {
            history: HashMap::new(), // Create HashMap to store history of RNG results for each ID
        }
    }

    pub fn calibrate(&mut self, player_id: i64, base_rng_result: i64) -> i64 {
        let rng_history = self.history.entry(player_id).or_insert(Vec::new()); // Get or insert history for ID
        
        // logics to calibrate the RNG result
        let calibrated_result = base_rng_result; // Placeholder for now

        rng_history.push(calibrated_result);
        calibrated_result
    }
}