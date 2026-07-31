use std::sync::Arc;

use airs_demo_core::AirsDemoCore;
use airs_gui::{Gui, GuiCreateInfo, GuiFrame};
use airs_window::{WgpuContext, WgpuWindow, WgpuWindowHandler, WindowEvent};
use winit::{dpi::LogicalSize, window::WindowAttributes};

use crate::main_ui_scene::MainUiScene;

pub struct MainWindow {
    _core: Arc<AirsDemoCore>,
    gui: Option<Gui>,
}

impl MainWindow {
    pub fn new(core: Arc<AirsDemoCore>) -> Self {
        Self {
            _core: core,
            gui: None,
        }
    }

    fn gui_mut(&mut self) -> &mut Gui {
        self.gui.as_mut().expect("gui must be initialized")
    }
}

impl WgpuWindowHandler for MainWindow {
    fn window_attributes(&self) -> WindowAttributes {
        WindowAttributes::default()
            .with_title(&self._core.config.window.title)
            .with_inner_size(LogicalSize::new(
                self._core.config.window.width,
                self._core.config.window.height,
            ))
    }

    fn max_frame_rate(&self) -> u32 {
        self._core.config.window.max_frame_rate
    }

    fn init(&mut self, wgpu_window: &WgpuWindow) -> anyhow::Result<()> {
        let inner_size = wgpu_window.inner_size();
        let mut gui = Gui::new(GuiCreateInfo {
            x: 0,
            y: 0,
            width: inner_size.width,
            height: inner_size.height,
            scale_factor: wgpu_window.scale_factor() as f32,
            texture_format: wgpu_window.surface_config().format,
            wgpu_adapter: wgpu_window.adapter().clone(),
            wgpu_device: wgpu_window.device().clone(),
            wgpu_queue: wgpu_window.queue().clone(),
        })?;
        gui.world_mut().set_root_entity(|_| MainUiScene::new());
        self.gui = Some(gui);
        Ok(())
    }

    fn update(&mut self) -> bool {
        let gui = self.gui_mut();
        gui.update();
        gui.needs_redraw()
    }

    fn resize(&mut self, _wgpu_ctx: &WgpuContext<'_>, width: u32, height: u32) {
        self.gui_mut().resize(width, height);
    }

    fn rescale(&mut self, _wgpu_ctx: &WgpuContext<'_>, scale_factor: f32) {
        self.gui_mut().rescale(scale_factor);
    }

    fn close(&mut self, _wgpu_ctx: &WgpuContext<'_>) {
        self.gui_mut().close();
    }

    fn event(&mut self, event: &WindowEvent) {
        self.gui_mut().window_event(event);
    }

    fn render(
        &mut self,
        _wgpu_ctx: &WgpuContext<'_>,
        texture_view: &airs_window::wgpu::TextureView,
        command_encoder: &mut airs_window::wgpu::CommandEncoder,
    ) -> anyhow::Result<()> {
        self.gui_mut().render(GuiFrame {
            texture_view,
            command_encoder,
        })
    }
}
