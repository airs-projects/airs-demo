use airs_gui::{Div, div, rgb};

pub struct AgentsView;

impl AgentsView {
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
                    .width(270.0)
                    .h_full()
                    .flex_none()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .height(52.0)
                            .flex()
                            .items_center()
                            .justify_between()
                            .border_bottom(1.0)
                            .border_color(rgb(0x25211a))
                            .child(div().text_small().text_color(rgb(0xd6b15f)).child("AGENTS"))
                            .child(small_button("Create")),
                    )
                    .child(
                        div()
                            .padding_y(12.0)
                            .flex()
                            .flex_col()
                            .gap(8.0)
                            .child(agent_row("Chat", "Running", true))
                            .child(agent_row("Test", "Idle", false)),
                    ),
            )
            .child(
                div()
                    .h_full()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .justify_between()
                    .padding(16.0)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(12.0)
                            .child(div().text_2xl().text_color(rgb(0xf1eadf)).child("Chat"))
                            .child(
                                div()
                                    .text_small()
                                    .text_color(rgb(0x8f8a82))
                                    .child("A local conversational agent."),
                            ),
                    )
                    .child(
                        div()
                            .w_full()
                            .rounded(3.0)
                            .background(rgb(0x151515))
                            .padding_x(16.0)
                            .padding_y(14.0)
                            .text_small()
                            .text_color(rgb(0x8f8a82))
                            .child("Message Chat…"),
                    ),
            )
    }
}

fn agent_row(name: &'static str, status: &'static str, active: bool) -> Div {
    let row = div()
        .w_full()
        .padding_x(10.0)
        .padding_y(10.0)
        .rounded(3.0)
        .flex()
        .flex_col()
        .gap(4.0)
        .child(name)
        .child(div().text_small().text_color(rgb(0x8f8a82)).child(status));

    if active {
        row.background(rgb(0x1b1812)).text_color(rgb(0xf8e7bd))
    } else {
        row.text_color(rgb(0xd7d0c8))
    }
}

fn small_button(label: &'static str) -> Div {
    div()
        .rounded(3.0)
        .background(rgb(0x151515))
        .padding_x(12.0)
        .padding_y(6.0)
        .text_small()
        .text_color(rgb(0xd7d0c8))
        .child(label)
}
