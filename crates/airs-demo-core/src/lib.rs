use std::sync::Arc;

pub mod assets;
pub mod config;
pub mod log;

pub struct AirsDemoCore {
    pub assets: assets::Assets,
    pub log: log::Log,
    pub config: config::Config,
}

impl AirsDemoCore {
    pub fn new() -> anyhow::Result<Arc<Self>> {
        let log = log::Log::new();
        tracing::info!(version = airs::version(), "airs-demo start");

        let assets = assets::Assets::new();
        let config = config::Config::new()?;

        Ok(Arc::new(Self {
            assets,
            log,
            config,
        }))
    }
}
