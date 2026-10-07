use dioxus::prelude::*;

use crate::components::music_player::MusicPlayer;
use crate::context::{AppSettingsProvider, FavoritesProvider, HomeProvider, PlaybackProvider};
use crate::route::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        AppSettingsProvider {
            HomeProvider {
                PlaybackProvider {
                    FavoritesProvider {
                        Router::<Route> {}
                        MusicPlayer {}
                    }
                }
            }
        }
    }
}
