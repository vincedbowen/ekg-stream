#[derive(Clone, Debug)]
pub struct Config {
    pub bind_addr: String,
    pub max_batch_samples: usize,
}

impl Config {
    pub fn from_env() -> Self {
        let bind_addr = std::env::var("INGEST_BIND").unwrap_or_else(|_| "127.0.0.1:50051".into());
        let max_batch_samples = std::env::var("INGEST_MAX_BATCH_SAMPLES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(2000);
        Self { bind_addr, max_batch_samples }
    }
}
