use std::sync::Arc;

use airs_demo_core::AirsDemoCore;
use airs_demo_gui::MainWindow;
use airs_window::WindowLoop;

fn create_core() -> anyhow::Result<Arc<AirsDemoCore>> {
    let log = airs_demo_core::log::Log::new();
    tracing::info!(version = airs::version(), "airs-demo start");

    let assets = airs_demo_core::assets::Assets::new();
    let config = airs_demo_core::config::Config::new()?;

    Ok(AirsDemoCore::new(assets, log, config))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let core = create_core()?;
    let main_window = MainWindow::new(core);
    WindowLoop::new().run(main_window)?;
    Ok(())
}
