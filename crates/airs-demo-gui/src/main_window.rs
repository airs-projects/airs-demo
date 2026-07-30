use std::sync::Arc;

use airs_demo_core::AirsDemoCore;
use airs_gui::{Gui, GuiCreateInfo, GuiFrame};
use airs_window::{WgpuContext, WgpuWindow, WgpuWindowHandler, WindowEvent};

use crate::main_ui_scene::MainUiScene;

pub struct MainWindow {
    _core: Arc<AirsDemoCore>,
    gui: Gui,
}

impl MainWindow {
    pub fn new(
        wgpu_window: &WgpuWindow,
        core: Arc<AirsDemoCore>,
        wake: Arc<dyn Fn() + Send + Sync>,
    ) -> anyhow::Result<Self> {
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
            wake,
        })?;
        gui.world_mut().set_root_entity(|_| MainUiScene::new());

        Ok(Self { _core: core, gui })
    }
}

impl WgpuWindowHandler for MainWindow {
    fn update(&mut self) -> bool {
        self.gui.update();
        self.gui.needs_redraw()
    }

    fn resize(&mut self, _wgpu_ctx: &WgpuContext<'_>, width: u32, height: u32) {
        self.gui.resize(width, height);
    }

    fn rescale(&mut self, _wgpu_ctx: &WgpuContext<'_>, scale_factor: f32) {
        self.gui.rescale(scale_factor);
    }

    fn close(&mut self, _wgpu_ctx: &WgpuContext<'_>) {
        self.gui.close();
    }

    fn event(&mut self, event: &WindowEvent) {
        self.gui.window_event(event);
    }

    fn render(
        &mut self,
        _wgpu_ctx: &WgpuContext<'_>,
        texture_view: &airs_window::wgpu::TextureView,
        command_encoder: &mut airs_window::wgpu::CommandEncoder,
    ) -> anyhow::Result<()> {
        self.gui.render(GuiFrame {
            texture_view,
            command_encoder,
        })
    }
}
