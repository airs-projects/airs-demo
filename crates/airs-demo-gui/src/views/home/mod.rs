use airs_gui::{Div, div, rgb};

pub struct HomeView;

impl HomeView {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self) -> Div {
        div()
            .size_full()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .gap(8.0)
            .text_color(rgb(0xf1eadf))
            .child(div().text_2xl().text_color(rgb(0xd6b15f)).child("Oasis"))
            .child(
                div()
                    .text_small()
                    .text_color(rgb(0x8f8a82))
                    .child("Local games, saves, and launch tools."),
            )
    }
}
