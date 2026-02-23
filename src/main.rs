use dioxus::prelude::*;

use crate::common::components::dock::Dock;
use crate::common::context::{AlertProvider, FavoritesProvider, PlaybackProvider};
use crate::favorite::FavoriteList;
use crate::home::Home;
use crate::setting::Setting;

mod common;
mod core;
mod favorite;
mod home;
mod setting;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    #[cfg(not(feature = "desktop"))]
    dioxus::launch(App);

    #[cfg(feature = "desktop")]
    {
        dioxus::LaunchBuilder::new()
            .with_cfg(
                dioxus::desktop::Config::default()
                    .with_menu(None)
                    .with_window(dioxus::desktop::WindowBuilder::new().with_title("doyou")),
            )
            .launch(App);
    }
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        AlertProvider {
            PlaybackProvider {
                FavoritesProvider { Router::<Route> {} }
            }
        }
    }
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(Dock)]
    #[route("/")]
    Home {},

    #[route("/favorite")]
    FavoriteList {},

    #[route("/setting")]
    Setting {},
}
