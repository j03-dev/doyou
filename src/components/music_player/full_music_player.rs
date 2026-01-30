use dioxus::prelude::*;

use super::{MusicController, ProgressBar};
use crate::{
    components::icons::{CloseIcon, FavoriteIcon},
    providers::Playback,
};

#[component]
pub fn FullMusicPlayer(on_close_full_player: EventHandler<()>) -> Element {
    // Changed prop name
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
        div { class: "fixed inset-0 z-50 bg-base-100 flex flex-col",
            button {
                class: "absolute top-4 right-4 z-10 btn btn-circle btn-ghost",
                onclick: move |_| on_close_full_player.call(()), // Changed handler call
                CloseIcon {}
            }
            div { class: "flex-1 flex flex-col px-6 pt-5 pb-8",
                div { class: "relative flex-1 flex flex-col items-center justify-center pt-12 md:pt-16 lg:pt-20 mb-8 lg:mb-12",
                    div { class: "relative w-64 h-64 md:w-80 md:h-80 lg:w-96 lg:h-96 mb-10",
                        div { class: "absolute -inset-10 bg-gradient-to-br from-primary/40 via-secondary/30 to-accent/20 blur-3xl rounded-full" }
                        img {
                            class: "relative w-full h-full object-cover rounded-3xl",
                            src: thumbnail,
                            alt: "Album cover",
                        }
                    }
                    div { class: "flex justify-between",
                        div { class: "mb-6 lg:mb-10",
                            h2 { class: "text-xl font-bold mb-2 lg:text-2xl lg:mb-3",
                                {title}
                            }
                            p { class: "text-lg opacity-60 font-medium", {artist} }
                        }
                        button { class: "btn btn-ghost", FavoriteIcon {} }
                    }
                    div { class: "w-full max-w-xl mx-auto",
                        ProgressBar { playback }
                    }
                    div { class: "mt-10 lg:mt-12",
                        div { class: "flex items-center justify-center gap-6 mb-8 lg:gap-10",
                            MusicController { playback }
                        }
                    }
                }
            }
        }
    }
}
