use std::sync::Arc;

use airs_gui::{Gui, GuiCreateInfo, GuiFrame};
use airs_window::{WgpuContext, WgpuWindow, WgpuWindowHandler, WindowEvent};

use crate::AppContext;
use crate::view::main_ui_scene::MainUiScene;

pub struct MainWindow {
    _app_context: Arc<AppContext>,
    gui: Gui,
}

impl MainWindow {
    pub fn new(wgpu_window: &WgpuWindow, app_context: Arc<AppContext>) -> anyhow::Result<Self> {
        let gui = Gui::new(
            GuiCreateInfo {
                window_id: wgpu_window.id(),
                inner_size: wgpu_window.inner_size(),
                scale_factor: wgpu_window.scale_factor() as f32,
                wgpu_instance: wgpu_window.instance().clone(),
                wgpu_adapter: wgpu_window.adapter().clone(),
                wgpu_device: wgpu_window.device().clone(),
                wgpu_queue: wgpu_window.queue().clone(),
            },
            |_cx| MainUiScene::new(),
        )?;

        Ok(Self {
            _app_context: app_context,
            gui,
        })
    }
}

impl WgpuWindowHandler for MainWindow {
    fn resize(&mut self, _wgpu_ctx: &WgpuContext<'_>, _width: u32, _height: u32) {}

    fn event(&mut self, event: &WindowEvent) {
        self.gui.window_event(event);
    }

    fn render(
        &mut self,
        wgpu_ctx: &WgpuContext<'_>,
        texture_view: &airs_window::wgpu::TextureView,
        command_encoder: &mut airs_window::wgpu::CommandEncoder,
        scale_factor: f32,
    ) -> anyhow::Result<()> {
        self.gui.render(GuiFrame {
            texture_view,
            command_encoder,
            format: wgpu_ctx.surface_config().format,
            width: wgpu_ctx.surface_config().width,
            height: wgpu_ctx.surface_config().height,
            scale_factor,
        })
    }
}
