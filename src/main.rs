use std::{error::Error, sync::Arc};

use airs_config::Config;
use airs_window::WindowContext;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{DeviceEvent, DeviceId, StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use crate::config::ConfigData;

mod assets;
mod config;

struct DemoApp {
    config: Config<ConfigData>,
    main_window: Option<Arc<Window>>,
    main_window_context: Option<WindowContext<'static>>,
}

impl DemoApp {
    fn new() -> airs_config::Result<Self> {
        let config = Config::<ConfigData>::new()?;

        Ok(Self {
            config,
            main_window: None,
            main_window_context: None,
        })
    }

    #[tracing::instrument(skip_all)]
    fn create_main_window(&mut self, event_loop: &ActiveEventLoop) -> airs_window::Result<()> {
        let window_config = &self.config.window;
        let attributes = WindowAttributes::default()
            .with_title(&window_config.title)
            .with_inner_size(LogicalSize::new(window_config.width, window_config.height));

        let main_window = Arc::new(event_loop.create_window(attributes)?);

        self.main_window = Some(main_window);
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    fn create_main_window_context(&mut self) -> airs_window::Result<()> {
        let main_window = self.main_window.as_ref().unwrap();
        let size = main_window.inner_size();
        let main_window_context = WindowContext::new(main_window.clone(), size.width, size.height)?;

        main_window.request_redraw();
        self.main_window_context = Some(main_window_context);
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

        if self.main_window_context.is_none()
            && let Err(error) = self.create_main_window_context()
        {
            tracing::error!(%error, "main window context create failed");
            event_loop.exit();
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
        let Some(main_window) = &self.main_window else {
            return;
        };
        if main_window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                tracing::info!(width = size.width, height = size.height, "window resize");
                if let Some(context) = &mut self.main_window_context {
                    context.resize(size.width, size.height);
                }
                main_window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                if let Some(context) = &self.main_window_context
                    && let Err(error) = context.render()
                {
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
        self.main_window_context = None;
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
