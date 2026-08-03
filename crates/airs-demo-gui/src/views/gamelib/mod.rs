use airs_gui::{Div, div, rgb};

const GAMES: [&str; 5] = [
    "Cyberpunk 2077",
    "ELDEN RING",
    "Baldur's Gate 3",
    "The Witcher 3",
    "Stardew Valley",
];

pub struct GameLibraryView;

impl GameLibraryView {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self) -> Div {
        div()
            .size_full()
            .flex()
            .flex_row()
            .gap(28.0)
            .child(self.sidebar())
            .child(self.detail())
    }

    fn sidebar(&self) -> Div {
        div()
            .width(260.0)
            .h_full()
            .flex()
            .flex_col()
            .flex_none()
            .border_color(rgb(0x25211a))
            .child(
                div()
                    .height(48.0)
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_bottom(1.0)
                    .border_color(rgb(0x25211a))
                    .padding_x(8.0)
                    .child(div().text_small().text_color(rgb(0xd6b15f)).child("GAMES"))
                    .child(
                        div()
                            .text_small()
                            .text_color(rgb(0x8f8a82))
                            .child("5 games"),
                    ),
            )
            .child(
                div().flex().flex_col().gap(4.0).padding_y(8.0).children(
                    GAMES
                        .iter()
                        .enumerate()
                        .map(|(index, game)| game_row(game, index == 0)),
                ),
            )
    }

    fn detail(&self) -> Div {
        div()
            .h_full()
            .flex_1()
            .flex()
            .flex_col()
            .gap(22.0)
            .padding(8.0)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap(24.0)
                    .child(
                        div()
                            .width(180.0)
                            .height(270.0)
                            .flex_none()
                            .rounded(3.0)
                            .background(rgb(0x101010))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_small()
                            .text_color(rgb(0x8f8a82))
                            .child("Game artwork"),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap(18.0)
                            .child(
                                div()
                                    .text_2xl()
                                    .text_color(rgb(0xf1eadf))
                                    .child("Cyberpunk 2077"),
                            )
                            .child(
                                div()
                                    .text_small()
                                    .text_color(rgb(0xd6b15f))
                                    .child("Cloud saves ready"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .gap(12.0)
                                    .child(primary_button("Launch"))
                                    .child(secondary_button("Sync Saves")),
                            ),
                    ),
            )
            .child(
                div()
                    .border_bottom(1.0)
                    .border_color(rgb(0x25211a))
                    .padding_y(16.0)
                    .flex()
                    .flex_col()
                    .gap(8.0)
                    .child(
                        div()
                            .text_small()
                            .text_color(rgb(0xd6b15f))
                            .child("INSTALL PATH"),
                    )
                    .child(
                        div()
                            .text_small()
                            .text_color(rgb(0xd7d0c8))
                            .child("C:\\Games\\Cyberpunk 2077"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(8.0)
                    .child(
                        div()
                            .text_small()
                            .text_color(rgb(0xd6b15f))
                            .child("SAVE PATHS"),
                    )
                    .child(
                        div()
                            .text_small()
                            .text_color(rgb(0xd7d0c8))
                            .child("Saved Games\\CD Projekt Red\\Cyberpunk 2077"),
                    ),
            )
    }
}

fn game_row(label: &'static str, active: bool) -> Div {
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
        row.text_color(rgb(0xd7d0c8))
    }
}

fn primary_button(label: &'static str) -> Div {
    div()
        .padding_x(20.0)
        .padding_y(9.0)
        .rounded(3.0)
        .background(rgb(0xd6b15f))
        .text_color(rgb(0x0b0b0d))
        .text_small()
        .child(label)
}

fn secondary_button(label: &'static str) -> Div {
    div()
        .padding_x(20.0)
        .padding_y(9.0)
        .rounded(3.0)
        .background(rgb(0x2f7f73))
        .text_color(rgb(0xf1eadf))
        .text_small()
        .child(label)
}
