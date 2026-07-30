use std::sync::Arc;

use airs_window::{ActiveWindowLoop, WindowLoop};
use winit::{dpi::LogicalSize, window::WindowAttributes};

use crate::view::main_window::MainWindow;

mod mods;
mod view;

pub(crate) struct AppContext {
    pub(crate) _assets: mods::assets::Assets,
    pub(crate) _log: mods::log::Log,
    pub(crate) config: mods::config::Config,
}

struct App {
    _context: Arc<AppContext>,
    _tokio_runtime: tokio::runtime::Runtime,
}

impl App {
    fn create_context() -> anyhow::Result<Arc<AppContext>> {
        let log = mods::log::Log::new();
        tracing::info!(version = airs::version(), "airs-demo start");

        let assets = mods::assets::Assets::new();
        let config = mods::config::Config::new()?;

        Ok(Arc::new(AppContext {
            _assets: assets,
            _log: log,
            config,
        }))
    }

    fn create_window(
        window_loop: &ActiveWindowLoop<'_>,
        app_context: Arc<AppContext>,
        tokio_handle: tokio::runtime::Handle,
    ) -> anyhow::Result<()> {
        let attributes = WindowAttributes::default()
            .with_title(&app_context.config.window.title)
            .with_inner_size(LogicalSize::new(
                app_context.config.window.width,
                app_context.config.window.height,
            ));

        let wake = window_loop.wake_callback();
        window_loop.create_wgpu_window(attributes, move |wgpu_window| {
            MainWindow::new(wgpu_window, app_context, tokio_handle, wake)
        })?;
        Ok(())
    }

    fn new(window_loop: &ActiveWindowLoop<'_>) -> anyhow::Result<Self> {
        let context = Self::create_context()?;
        let tokio_runtime = tokio::runtime::Runtime::new()?;
        Self::create_window(window_loop, context.clone(), tokio_runtime.handle().clone())?;

        Ok(Self {
            _context: context,
            _tokio_runtime: tokio_runtime,
        })
    }
}

fn main() -> anyhow::Result<()> {
    WindowLoop::new().run(|window_loop| App::new(window_loop))?;
    Ok(())
}
