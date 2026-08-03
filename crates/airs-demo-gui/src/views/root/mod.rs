use airs_gui::{Context, IntoElement, Render, div, rgb};

use super::{
    agents::AgentsView,
    gamelib::GameLibraryView,
    home::HomeView,
    navbar::{Navbar, Route},
    settings::SettingsView,
};

pub struct RootView {
    route: Route,
    navbar: Navbar,
    home: HomeView,
    game_library: GameLibraryView,
    agents: AgentsView,
    settings: SettingsView,
}

impl RootView {
    pub fn new() -> Self {
        let route = Route::Home;
        Self {
            route,
            navbar: Navbar::new(),
            home: HomeView::new(),
            game_library: GameLibraryView::new(),
            agents: AgentsView::new(),
            settings: SettingsView::new(),
        }
    }

    pub(super) fn set_route(&mut self, route: Route, cx: &mut Context<Self>) {
        if self.route != route {
            self.route = route;
            cx.notify();
        }
    }
}

impl Render for RootView {
    fn render(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match self.route {
            Route::Home => self.home.render(),
            Route::GameLibrary => self.game_library.render(),
            Route::Agents => self.agents.render(),
            Route::Settings => self.settings.render(),
        };

        div()
            .size_full()
            .flex()
            .flex_col()
            .background(rgb(0x0b0b0d))
            .text_color(rgb(0xf1eadf))
            .child(self.navbar.render(self.route, cx))
            .child(
                div()
                    .w_full()
                    .flex_1()
                    .overflow_hidden()
                    .padding(20.0)
                    .child(content),
            )
    }
}
