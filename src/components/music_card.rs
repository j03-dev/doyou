use dioxus::prelude::*;

use crate::{providers::Playback, types::Item};

#[component]
pub fn MusicCard(item: Item, index: usize) -> Element {
    let mut favorite = use_signal(|| false);
    let mut playback = use_context::<Playback>();

    let is_loading =
        use_memo(move || *playback.current_index.read() == index && *playback.is_loading.read());

    let is_playing_now =
        use_memo(move || *playback.current_index.read() == index && *playback.is_playing.read());

    rsx! {
        li { class: if is_playing_now() { "list-row bg-secondary text-base-content" } else { "list-row" },
            div { onclick: move |_| playback.start(index),
                img {
                    class: "size-30 rounded-box",
                    src: item.snippet.thumbnails.unwrap().medium.unwrap().url,
                }
            }
            div {
                div { {item.snippet.title} }
                div { class: "text-xs uppercase font-semibold opacity-60",
                    {item.snippet.channel_title}
                }
                p { class: "list-col-wrap line-clamp-3 text-xs mt-2", {item.snippet.description} }
                if is_loading() {
                    span { class: "loading loading-dots loading-sm" }
                }
            }
            button { class: "btn btn-ghost",
                svg {
                    class: "size-[1.2em]",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",

                    g {
                        stroke_linejoin: "round",
                        stroke_linecap: "round",
                        stroke_width: "2",
                        fill: "none",
                        stroke: "currentColor",
                        path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                        polyline { points: "7 10 12 15 17 10" }
                        line {
                            x1: "12",
                            y1: "15",
                            x2: "12",
                            y2: "3",
                        }
                    }
                }
            }
            button {
                class: "btn btn-ghost",
                onclick: move |_| favorite.set(!favorite()),
                svg {
                    class: "size-[1.2em]",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    g {
                        stroke_linejoin: "round",
                        stroke_linecap: "round",
                        stroke_width: "2",
                        fill: "none",
                        stroke: "currentColor",
                        class: {
                            if favorite() {
                                if is_playing_now() {
                                    "fill-white stroke-white"
                                } else {
                                    "fill-red-500 stroke-red-5000"
                                }
                            } else {
                                "fill-transparensakalava  t stroke-current"
                            }
                        },
                        path { d: "M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" }
                    }
                }
            }
        }
    }
}
