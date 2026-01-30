use dioxus::prelude::*;

use crate::components::music_player::{MusicController, ProgressBar};
use crate::providers::Playback;
use crate::components::icons::UpArrowIcon;

#[component]
pub fn MiniMusicPlayer(on_open_full_player: EventHandler<()>) -> Element {
    let playback = use_context::<Playback>();

    let thumbnail = playback
        .playing
        .read()
        .as_ref()
        .and_then(|i| {
            i.snippet
                .thumbnails
                .as_ref()?
                .high
                .as_ref()
                .map(|t| t.url.clone())
        })
        .unwrap_or("https://via.placeholder.com/300".to_string());

    let title = playback
        .playing
        .read()
        .as_ref()
        .map(|i| i.snippet.title.clone())
        .unwrap_or("Unknown title".to_string());

    let artist = playback
        .playing
        .read()
        .as_ref()
        .map(|i| i.snippet.channel_title.clone().unwrap_or_default())
        .unwrap_or("Unknown artist".to_string());

    rsx! {
        div { class: "bg-base-200",
            div { class: "p-3",
                div { class: "flex items-center justify-between gap-4",
                    div { class: "flex items-center gap-3 flex-1 min-w-0",
                        img {
                            class: "w-12 h-12 rounded-lg flex-shrink-0",
                            src: thumbnail,
                        }
                        div { class: "min-w-0 flex-1",
                            h3 { class: "font-semibold truncate text-sm", {title} }
                            p { class: "text-xs opacity-60 truncate", {artist} }
                        }
                    }
                    div { class: "flex justify-center items-center flex-1",
                        MusicController { playback }
                    }
                    div { class: "flex justify-end items-center flex-1 gap-4",
                        div { class: "w-2/3",
                            ProgressBar { playback }
                        }
                        button {
                            class: "btn btn-circle btn-ghost",
                            onclick: move |_| on_open_full_player.call(()),
                            UpArrowIcon {}
                        }
                    }
                }
            }
        }
    }
}
