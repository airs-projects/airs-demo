use std::sync::Arc;

use airs_demo_core::AirsDemoCore;
use airs_demo_gui::MainWindow;
use airs_window::{ActiveWindowLoop, WindowLoop};
use winit::{dpi::LogicalSize, window::WindowAttributes};

fn create_core() -> anyhow::Result<Arc<AirsDemoCore>> {
    let log = airs_demo_core::log::Log::new();
    tracing::info!(version = airs::version(), "airs-demo start");

    let assets = airs_demo_core::assets::Assets::new();
    let config = airs_demo_core::config::Config::new()?;

    Ok(AirsDemoCore::new(assets, log, config))
}

fn create_window(
    window_loop: &ActiveWindowLoop<'_>,
    core: Arc<AirsDemoCore>,
) -> anyhow::Result<()> {
    let attributes = WindowAttributes::default()
        .with_title(&core.config.window.title)
        .with_inner_size(LogicalSize::new(
            core.config.window.width,
            core.config.window.height,
        ));

    let wake = window_loop.wake_callback();
    window_loop.create_wgpu_window(attributes, move |wgpu_window| {
        MainWindow::new(wgpu_window, core, wake)
    })?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let core = create_core()?;
    WindowLoop::new().run(|window_loop| create_window(window_loop, core))?;
    Ok(())
}
