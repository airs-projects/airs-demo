use std::error::Error;

use airs::gui::{
    GuiFrame, GuiGpuContext, GuiLayer, GuiOptions,
    gpui::{Context, IntoElement, Render, Window, div, prelude::*, px, rgb},
    wgpu,
};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowId},
};

struct DemoView;

impl Render for DemoView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0x111827))
            .text_color(rgb(0xf9fafb))
            .text_size(px(24.0))
            .child(format!(
                "airs {} / airs-gui {}",
                airs::version(),
                airs::gui::version()
            ))
    }
}

struct App {
    state: AppState,
}

enum AppState {
    New,
    Running(ReadyApp),
    Suspended,
}

struct ReadyApp {
    window: WinitWindow,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    gui: GuiLayer,
}

impl App {
    fn new() -> Self {
        Self {
            state: AppState::New,
        }
    }
}

impl ReadyApp {
    fn new(event_loop: &ActiveEventLoop) -> Result<Self, Box<dyn Error>> {
        let window = event_loop.create_window(
            WinitWindow::default_attributes()
                .with_title("airs-demo")
                .with_inner_size(LogicalSize::new(1280, 720)),
        )?;

        let gpu = init_wgpu(&window)?;
        let gui_gpu = GuiGpuContext {
            instance: gpu.instance.clone(),
            adapter: gpu.adapter.clone(),
            device: gpu.device.clone(),
            queue: gpu.queue.clone(),
        };
        let gui = GuiLayer::new(&window, gui_gpu, GuiOptions::default(), |_, _| DemoView)?;

        Ok(Self {
            window,
            surface: gpu.surface,
            surface_config: gpu.surface_config,
            device: gpu.device,
            queue: gpu.queue,
            gui,
        })
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if window_id != self.window.id() {
            return;
        }

        match &event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                self.resize(size.width, size.height);
                self.gui.window_event(&event);
            }
            WindowEvent::RedrawRequested => {
                if let Err(error) = self.render() {
                    eprintln!("render failed: {error}");
                    event_loop.exit();
                }
            }
            _ => self.gui.window_event(&event),
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.gui.update();
        if self.gui.needs_redraw() {
            self.window.request_redraw();
        }

        event_loop.set_control_flow(ControlFlow::Wait);
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    fn render(&mut self) -> Result<(), Box<dyn Error>> {
        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame)
            | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
            wgpu::CurrentSurfaceTexture::Lost | wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.surface_config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => return Ok(()),
        };

        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
            format: Some(self.surface_config.format),
            ..Default::default()
        });
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("airs-demo encoder"),
            });

        {
            let _clear = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("airs-demo clear"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });
        }

        self.gui.render(GuiFrame {
            texture_view: &view,
            command_encoder: &mut encoder,
            format: self.surface_config.format,
            width: self.surface_config.width,
            height: self.surface_config.height,
        })?;

        self.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if matches!(self.state, AppState::New | AppState::Suspended) {
            self.state = match ReadyApp::new(event_loop) {
                Ok(app) => AppState::Running(app),
                Err(error) => {
                    eprintln!("startup failed: {error}");
                    event_loop.exit();
                    AppState::Suspended
                }
            };
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.state = AppState::Suspended;
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let AppState::Running(app) = &mut self.state {
            app.window_event(event_loop, window_id, event);
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let AppState::Running(app) = &mut self.state {
            app.about_to_wait(event_loop);
        }
    }
}

struct DemoGpu {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
}

fn init_wgpu(window: &WinitWindow) -> Result<DemoGpu, Box<dyn Error>> {
    let size = window.inner_size();
    let raw_window_handle = window.window_handle()?.as_raw();
    let raw_display_handle = window.display_handle()?.as_raw();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: select_backends(),
        flags: wgpu::InstanceFlags::default(),
        memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
        display: None,
        backend_options: wgpu::BackendOptions::default(),
    });

    let surface = unsafe {
        instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
            raw_display_handle: Some(raw_display_handle),
            raw_window_handle,
        })?
    };

    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }))?;

    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("airs-demo device"),
        required_features: wgpu::Features::empty(),
        required_limits: adapter.limits(),
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        memory_hints: wgpu::MemoryHints::default(),
        trace: wgpu::Trace::Off,
    }))?;

    let caps = surface.get_capabilities(&adapter);
    let format = select_surface_format(&caps).ok_or("surface has no supported format")?;
    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width.max(1),
        height: size.height.max(1),
        present_mode: select_present_mode(&caps),
        desired_maximum_frame_latency: 2,
        alpha_mode: select_alpha_mode(&caps),
        view_formats: Vec::new(),
    };

    surface.configure(&device, &surface_config);

    Ok(DemoGpu {
        instance,
        adapter,
        device,
        queue,
        surface,
        surface_config,
    })
}

fn select_backends() -> wgpu::Backends {
    if cfg!(target_os = "windows") {
        wgpu::Backends::DX12
    } else {
        wgpu::Backends::all()
    }
}

fn select_surface_format(caps: &wgpu::SurfaceCapabilities) -> Option<wgpu::TextureFormat> {
    caps.formats
        .iter()
        .copied()
        .find(|format| {
            *format == wgpu::TextureFormat::Rgba8UnormSrgb
                || *format == wgpu::TextureFormat::Bgra8UnormSrgb
        })
        .or_else(|| caps.formats.first().copied())
}

fn select_present_mode(caps: &wgpu::SurfaceCapabilities) -> wgpu::PresentMode {
    [
        wgpu::PresentMode::AutoVsync,
        wgpu::PresentMode::FifoRelaxed,
        wgpu::PresentMode::Fifo,
    ]
    .into_iter()
    .find(|mode| caps.present_modes.contains(mode))
    .unwrap_or(wgpu::PresentMode::Fifo)
}

fn select_alpha_mode(caps: &wgpu::SurfaceCapabilities) -> wgpu::CompositeAlphaMode {
    [
        wgpu::CompositeAlphaMode::Auto,
        wgpu::CompositeAlphaMode::Opaque,
        wgpu::CompositeAlphaMode::PreMultiplied,
    ]
    .into_iter()
    .find(|mode| caps.alpha_modes.contains(mode))
    .unwrap_or(wgpu::CompositeAlphaMode::Auto)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("airs {}", airs::version());

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}
