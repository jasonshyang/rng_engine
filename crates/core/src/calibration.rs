use std::collections::HashMap;

pub struct Calibration {
    history: HashMap<u64, Vec<u32>>,
}

impl Calibration {
    pub fn new() -> Self {
        Calibration {
            history: HashMap::new(), // Create HashMap to store history of RNG results for each ID
        }
    }

    pub fn calibrate(&mut self, id: u64, base_rng_result: u32) -> u32 {
        let rng_history = self.history.entry(id).or_insert(Vec::new()); // Get or insert history for ID
        
        // logics to calibrate the RNG result
        let calibrated_result = base_rng_result; // Placeholder for now

        rng_history.push(calibrated_result);
        calibrated_result
    }
}