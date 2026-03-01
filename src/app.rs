use dioxus::prelude::*;

use crate::common::context::{AppSettingsProvider, FavoritesProvider, PlaybackProvider};
use crate::route::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        AppSettingsProvider {
            PlaybackProvider {
                FavoritesProvider { Router::<Route> {} }
            }
        }
    }
}
