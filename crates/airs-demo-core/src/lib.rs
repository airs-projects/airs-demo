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
    pub fn new(assets: assets::Assets, log: log::Log, config: config::Config) -> Arc<Self> {
        Arc::new(Self {
            assets,
            log,
            config,
        })
    }
}
