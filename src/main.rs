use std::error::Error;

use airs_config::Config;
use airs_window::WgpuWindow;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{DeviceEvent, DeviceId, StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{WindowAttributes, WindowId},
};

use crate::config::ConfigData;

mod assets;
mod config;

struct DemoApp {
    config: Config<ConfigData>,
    main_window: Option<WgpuWindow>,
}

impl DemoApp {
    fn new() -> airs_config::Result<Self> {
        let config = Config::<ConfigData>::new()?;

        Ok(Self {
            config,
            main_window: None,
        })
    }

    #[tracing::instrument(skip_all)]
    fn create_main_window(&mut self, event_loop: &ActiveEventLoop) -> airs_window::Result<()> {
        let window_config = &self.config.window;
        let attributes = WindowAttributes::default()
            .with_title(&window_config.title)
            .with_inner_size(LogicalSize::new(window_config.width, window_config.height));

        let mut main_window = WgpuWindow::new(event_loop, attributes)?;
        main_window.init_wgpu()?;

        self.main_window = Some(main_window);
        Ok(())
    }
}

impl ApplicationHandler for DemoApp {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        tracing::trace!(?cause, "new events");
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        tracing::info!("application resumed");

        if self.main_window.is_none()
            && let Err(error) = self.create_main_window(event_loop)
        {
            tracing::error!(%error, "main window create failed");
            event_loop.exit();
            return;
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, _event: ()) {
        tracing::trace!("user event");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(main_window) = &mut self.main_window else {
            return;
        };
        if main_window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                tracing::info!(width = size.width, height = size.height, "window resize");
                main_window.resize(size.width, size.height);
                main_window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                if let Err(error) = main_window.render() {
                    tracing::error!(%error, "window render failed");
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        device_id: DeviceId,
        event: DeviceEvent,
    ) {
        tracing::trace!(?device_id, ?event, "device event");
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        tracing::trace!("about to wait");
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        tracing::info!("application suspended");
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        tracing::info!("application exiting");
        self.main_window = None;
    }

    fn memory_warning(&mut self, _event_loop: &ActiveEventLoop) {
        tracing::warn!("memory warning");
    }
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_max_level(tracing::Level::INFO)
        .with_target(true)
        .with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::ENTER
                | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
        )
        .compact()
        .init();
}

fn main() -> Result<(), Box<dyn Error>> {
    init_tracing();
    tracing::info!(version = airs::version(), "airs-demo start");

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = DemoApp::new()?;
    event_loop.run_app(&mut app)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_app_initializes_config() {
        let app = DemoApp::new().unwrap();

        assert_eq!(app.config.window.title, "airs-demo");
        assert_eq!(app.config.window.width, 1280);
        assert_eq!(app.config.window.height, 720);
    }
}
