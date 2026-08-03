use airs_gui::{Div, div, rgb};

pub struct SettingsView;

impl SettingsView {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self) -> Div {
        div()
            .size_full()
            .flex()
            .flex_row()
            .gap(28.0)
            .child(
                div()
                    .width(220.0)
                    .h_full()
                    .flex_none()
                    .flex()
                    .flex_col()
                    .gap(6.0)
                    .child(setting_nav("Application", true))
                    .child(setting_nav("User Data", false))
                    .child(setting_nav("Logging", false))
                    .child(setting_nav("Game Library", false))
                    .child(setting_nav("Cloud Saves", false)),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap(22.0)
                    .child(
                        div()
                            .text_2xl()
                            .text_color(rgb(0xf1eadf))
                            .child("Application"),
                    )
                    .child(setting_field("Identifier", "oasis"))
                    .child(setting_field("Title", "Oasis")),
            )
    }
}

fn setting_nav(label: &'static str, active: bool) -> Div {
    let row = div()
        .w_full()
        .padding_x(12.0)
        .padding_y(9.0)
        .rounded(3.0)
        .text_small()
        .child(label);

    if active {
        row.background(rgb(0x1b1812)).text_color(rgb(0xf8e7bd))
    } else {
        row.text_color(rgb(0xa8a29a))
    }
}

fn setting_field(label: &'static str, value: &'static str) -> Div {
    div()
        .w_full()
        .border_bottom(1.0)
        .border_color(rgb(0x25211a))
        .padding_y(14.0)
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .child(div().text_small().text_color(rgb(0xd7d0c8)).child(label))
        .child(
            div()
                .width(320.0)
                .rounded(3.0)
                .background(rgb(0x151515))
                .padding_x(12.0)
                .padding_y(8.0)
                .text_small()
                .text_color(rgb(0xf1eadf))
                .child(value),
        )
}
