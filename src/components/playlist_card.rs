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
            class: "carousel-item card bg-base-100 w-36 shrink-0 transition-colors hover:bg-base-200/60 sm:w-40",
            CoverCollage { thumbnails, class: "rounded-xl" }
            div { class: "px-1 pt-2 pb-1",
                h2 {
                    class: "truncate text-sm font-semibold leading-tight",
                    title: "{name}",
                    "{name}"
                }
                p { class: "truncate text-xs text-base-content/60",
                    "{count} "
                    if count == 1 { "song" } else { "songs" }
                }
            }
        }
    }
}
