use airs_gui::{Context, IntoElement, Render, Window as GpuiWindow};

pub struct MainUiScene;

impl MainUiScene {
    pub fn new() -> Self {
        Self
    }
}

impl Render for MainUiScene {
    fn render(&mut self, _window: &mut GpuiWindow, _cx: &mut Context<Self>) -> impl IntoElement {
        "Hello World"
    }
}
