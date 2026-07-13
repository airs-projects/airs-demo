use airs_window::WindowLoop;
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
    context: AppContext,
    window_loop: WindowLoop,
}

impl App {
    fn new() -> anyhow::Result<Self> {
        let log = mods::log::Log::new();
        tracing::info!(version = airs::version(), "airs-demo start");

        let assets = mods::assets::Assets::new();
        let config = mods::config::Config::new()?;

        Ok(Self {
            context: AppContext {
                _assets: assets,
                _log: log,
                config,
            },
            window_loop: WindowLoop::new(),
        })
    }

    fn run(mut self) -> airs_window::Result<()> {
        let attributes = WindowAttributes::default()
            .with_title(&self.context.config.window.title)
            .with_inner_size(LogicalSize::new(
                self.context.config.window.width,
                self.context.config.window.height,
            ));
        let context = self.context;

        self.window_loop.run(move |window_loop| {
            window_loop.create_wgpu_window(attributes, MainWindow::new(context))
        })
    }
}

fn main() -> anyhow::Result<()> {
    App::new()?.run()?;
    Ok(())
}
