use airs_gui::{Gui, GuiGpuResources};
use airs_window::{Result, WgpuContext, WgpuWindowHandler};

use crate::AppContext;

pub struct MainWindow {
    _app_ctx: AppContext,
    gui: Option<Gui>,
}

impl MainWindow {
    pub fn new(app_ctx: AppContext) -> Self {
        Self {
            _app_ctx: app_ctx,
            gui: None,
        }
    }

    fn init_gui(&mut self, wgpu_ctx: &WgpuContext<'_>) {
        let resources = GuiGpuResources::new(
            wgpu_ctx.instance().clone(),
            wgpu_ctx.adapter().clone(),
            wgpu_ctx.device().clone(),
            wgpu_ctx.queue().clone(),
        );

        self.gui = Some(Gui::new(resources));
    }
}

impl WgpuWindowHandler for MainWindow {
    fn init(&mut self, wgpu_ctx: &WgpuContext<'_>) -> Result<()> {
        self.init_gui(wgpu_ctx);
        Ok(())
    }

    fn resize(&mut self, _wgpu_ctx: &WgpuContext<'_>, _width: u32, _height: u32) {}

    fn render(
        &mut self,
        _wgpu_ctx: &WgpuContext<'_>,
        _texture_view: &airs_window::wgpu::TextureView,
        _command_encoder: &mut airs_window::wgpu::CommandEncoder,
    ) -> Result<()> {
        let _ = self.gui.as_ref().expect("gui must be initialized").gpu();
        Ok(())
    }
}
