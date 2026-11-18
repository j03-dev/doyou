use dioxus::prelude::*;

use crate::components::cover::CoverCollage;
use crate::route::Route;

#[component]
pub fn PlaylistCard(
    name: String,
    #[props(default)] thumbnails: Vec<String>,
    #[props(default)] count: usize,
    to: Route,
) -> Element {
    rsx! {
        Link {
            to,
            class: "carousel-item card w-36 shrink-0 rounded-2xl border border-base-content/10 bg-base-100 p-3 shadow-sm transition-transform hover:-translate-y-0.5 sm:w-40",
            CoverCollage { thumbnails, class: "rounded-xl" }
            div { class: "px-0.5 pt-2 pb-1",
                h2 {
                    class: "truncate text-sm font-bold leading-tight",
                    title: "{name}",
                    "{name}"
                }
                p { class: "truncate text-xs text-base-content/60",
                    "{count} "
                    if count == 1 {
                        "song"
                    } else {
                        "songs"
                    }
                }
            }
        }
    }
}
