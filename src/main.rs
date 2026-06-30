use std::error::Error;

use airs::{
    gui::{
        GuiLayer, GuiOptions,
        gpui::{Context, IntoElement, Render, Window as GuiWindow, div, prelude::*, px, rgb},
    },
    window::{self, LogicalSize, WindowAttributes, WindowContext, WindowEvent, WindowSize},
};

struct DemoView;

impl Render for DemoView {
    fn render(&mut self, _window: &mut GuiWindow, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0x111827))
            .text_color(rgb(0xf9fafb))
            .text_size(px(24.0))
            .child(format!(
                "airs {} / airs-gui {} / airs-window {}",
                airs::version(),
                airs::gui::version(),
                airs::window::version()
            ))
    }
}

#[derive(Default)]
struct MainWindowHandler {
    gui: Option<GuiLayer>,
}

impl window::WindowHandler for MainWindowHandler {
    fn init(&mut self, cx: &mut WindowContext<'_>) -> window::Result<()> {
        self.gui = Some(cx.create_gui_layer(GuiOptions::default(), |_, _| DemoView)?);
        Ok(())
    }

    fn event(&mut self, cx: &mut WindowContext<'_>, event: WindowEvent) -> window::Result<()> {
        if matches!(&event, WindowEvent::CloseRequested) {
            cx.exit();
            return Ok(());
        }

        if let Some(gui) = &mut self.gui {
            cx.dispatch_event_to_gui(&event, gui);
        }
        Ok(())
    }

    fn resize(
        &mut self,
        cx: &mut WindowContext<'_>,
        _size: WindowSize,
        event: WindowEvent,
    ) -> window::Result<()> {
        if let Some(gui) = &mut self.gui {
            cx.dispatch_event_to_gui(&event, gui);
        }
        Ok(())
    }

    fn redraw(&mut self, cx: &mut WindowContext<'_>) -> window::Result<()> {
        if let Some(gui) = &mut self.gui {
            cx.render_gui(gui)?;
        }
        Ok(())
    }

    fn update(&mut self, cx: &mut WindowContext<'_>) -> window::Result<()> {
        if let Some(gui) = &mut self.gui {
            gui.update();
            if gui.needs_redraw() {
                cx.request_redraw();
            }
        }

        cx.set_wait();
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("airs {}", airs::version());

    let app = window::WindowApp::new()?;
    app.run(|cx| {
        let mut window = cx.create_window(
            WindowAttributes::default()
                .with_title("airs-demo")
                .with_inner_size(LogicalSize::new(1280, 720)),
        )?;
        window.set_handler(MainWindowHandler::default())?;
        Ok(window)
    })?;
    Ok(())
}
