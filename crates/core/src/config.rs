use serde::Deserialize;
use toml;

#[derive(Deserialize, Debug)]
pub struct CalibrationConfig {
    pub target_mean: i64,
    pub min: i64,
    pub max: i64,
    pub outlier_adj_factor: f64, // this is a placeholder for now for testing
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub calibration: CalibrationConfig,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let file = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&file).unwrap();
        Ok(config)
    }
}