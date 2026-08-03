use airs_gui::{Context, Div, div, rgb};

use super::root::RootView;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Home,
    GameLibrary,
    Agents,
    Settings,
}

struct NavItem {
    id: &'static str,
    label: &'static str,
    route: Route,
}

pub struct Navbar {
    items: [NavItem; 4],
}

impl Navbar {
    pub fn new() -> Self {
        Self {
            items: [
                NavItem {
                    id: "main-nav-home",
                    label: "Home",
                    route: Route::Home,
                },
                NavItem {
                    id: "main-nav-library",
                    label: "Library",
                    route: Route::GameLibrary,
                },
                NavItem {
                    id: "main-nav-agents",
                    label: "Agents",
                    route: Route::Agents,
                },
                NavItem {
                    id: "main-nav-settings",
                    label: "Settings",
                    route: Route::Settings,
                },
            ],
        }
    }

    pub fn render(&self, selected: Route, cx: &Context<RootView>) -> Div {
        div()
            .w_full()
            .height(58.0)
            .flex()
            .flex_row()
            .flex_none()
            .items_center()
            .justify_start()
            .background(rgb(0x0b0b0d))
            .border_bottom(1.0)
            .border_color(rgb(0x25211a))
            .padding_x(24.0)
            .gap(24.0)
            .text_color(rgb(0xf1eadf))
            .child(div().text_large().text_color(rgb(0xd6b15f)).child("Oasis"))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(8.0)
                    .children(self.items.iter().map(|item| {
                        let route = item.route;
                        nav_item(item, route == selected)
                            .id(item.id)
                            .on_click(cx.listener(move |root, _event, cx| {
                                root.set_route(route, cx);
                            }))
                    })),
            )
    }
}

fn nav_item(item: &NavItem, active: bool) -> Div {
    let element = div()
        .padding_x(16.0)
        .padding_y(6.0)
        .rounded(3.0)
        .text_small()
        .child(item.label);

    if active {
        element.background(rgb(0x1b1812)).text_color(rgb(0xf8e7bd))
    } else {
        element.text_color(rgb(0xa8a29a))
    }
}
