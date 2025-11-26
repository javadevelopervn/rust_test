mod config;
mod runner;

use config::TestConfig;
use runner::run_test;
use std::fs;
use serde_yaml;

#[tokio::main]
async fn main() {
    let config_str = fs::read_to_string("config.yml").expect("Failed to read config file");
    let test_config: TestConfig = serde_yaml::from_str(&config_str).expect("Failed to parse config");

    run_test(test_config).await;
}