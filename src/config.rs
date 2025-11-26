use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct TestConfig {
    pub steps: Vec<TestStep>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TestStep {
    pub name: String,
    pub request: Request,
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
    #[serde(default = "default_iterations")]
    pub iterations: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Request {
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub headers: Vec<(String, String)>,
    #[serde(default)]
    pub body: String,
}

fn default_concurrency() -> usize {
    1
}

fn default_iterations() -> usize {
    1
}