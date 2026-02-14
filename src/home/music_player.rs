use dioxus::prelude::*;

use crate::common::components::button::IconButton;
use crate::common::components::icons::{
    CloseIcon, FavoriteIcon, NextIcon, PauseIcon, PlayIcon, PrevIcon,
};
use crate::common::components::loading::LoadingSpinner;
use crate::core::playback::Playback;

#[component]
pub fn MusicPlayer() -> Element {
    let mut show_full_player = use_signal(|| true);
    rsx! {
        if show_full_player() {
            FullMusicPlayer { on_close_full_player: move |_| show_full_player.set(false) }
        } else {
            div { class: "fixed bottom-0 left-0 w-full bg-base-200 shadow-inner z-50",
                MiniMusicPlayer { on_open_full_player: move |_| show_full_player.set(true) }
            }
        }
    }
}

#[component]
fn FullMusicPlayer(on_close_full_player: EventHandler<MouseEvent>) -> Element {
    let playback = use_context::<Playback>();

    let playing = playback.playing.read();

    let thumbnail = playing
        .as_ref()
        .map(|i| i.snippet.thumbnails.default.url.clone())
        .unwrap_or_default();

    let title = playing
        .as_ref()
        .map(|i| i.snippet.title.clone())
        .unwrap_or("Unknown title".to_string());

    let artist = playing
        .as_ref()
        .map(|i| i.snippet.channel_title.clone())
        .unwrap_or("Unknown artist".to_string());

    rsx! {
        div { class: "fixed inset-0 z-50 bg-base-100 flex flex-col",
            IconButton {
                class: "absolute top-4 right-4 z-10t",
                on_click: on_close_full_player,
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
                            h2 {
                                class: "text-xl font-bold mb-2 lg:text-2xl lg:mb-3",
                                dangerous_inner_html: title,
                            }
                            p {
                                class: "text-lg opacity-60 font-medium",
                                dangerous_inner_html: artist,
                            }
                        }
                        IconButton { FavoriteIcon {} }
                    }
                    div { class: "w-full max-w-xl mx-auto",
                        ProgressBar { playback }
                    }
                    div { class: "mt-10 lg:mt-12 mb-8",
                        MusicController { playback }
                    }
                }
            }
        }
    }
}

#[component]
fn MiniMusicPlayer(on_open_full_player: EventHandler<()>) -> Element {
    let playback = use_context::<Playback>();

    let playing = playback.playing.read();

    let thumbnail = playing
        .as_ref()
        .map(|i| i.snippet.thumbnails.default.url.clone())
        .unwrap_or_default();

    let title = playing
        .as_ref()
        .map(|i| i.snippet.title.clone())
        .unwrap_or("Unknown title".to_string());

    let artist = playing
        .as_ref()
        .map(|i| i.snippet.channel_title.clone())
        .unwrap_or("Unknown artist".to_string());

    rsx! {
        div { class: "bg-base-200",
            div { class: "p-3",
                div { class: "flex items-center justify-between gap-4",
                    div {
                        class: "flex items-center gap-3 min-w-0 md:flex-1",
                        onclick: move |_| on_open_full_player.call(()),
                        img {
                            class: "w-12 h-12 rounded-lg flex-shrink-0",
                            src: thumbnail,
                        }
                        div { class: "min-w-0",
                            h3 {
                                class: "font-semibold truncate text-sm",
                                dangerous_inner_html: title,
                            }
                            p {
                                class: "text-xs opacity-60 truncate",
                                dangerous_inner_html: artist,
                            }
                        }
                    }
                    div { class: "flex justify-center flex-1",
                        MusicController { playback }
                    }
                    div { class: "hidden md:flex md:flex-1 md:justify-end",
                        div { class: "w-64 lg:w-80",
                            ProgressBar { playback }
                        }
                    }
                }
                div { class: "mt-3 w-full md:hidden",
                    ProgressBar { playback }
                }
            }
        }
    }
}

#[component]
fn MusicController(mut playback: Playback) -> Element {
    rsx! {
        div { class: "flex items-center justify-center gap-6 lg:gap-10",
            IconButton {
                class: "btn-secondary",
                on_click: move |_| playback.playback_controller(-1),
                PrevIcon {}
            }
            if !*playback.is_loading.read() {
                IconButton {
                    class: "btn-primary btn-xl",
                    on_click: move |_| playback.toggle_play(),
                    if *playback.is_playing.read() {
                        PlayIcon {}
                    } else {
                        PauseIcon {}
                    }
                }
            } else {
                IconButton { class: "btn-primary btn-xl", LoadingSpinner {} }
            }
            IconButton {
                class: "btn-secondary",
                on_click: move |_| playback.playback_controller(1),
                NextIcon {}
            }
        }
    }
}

#[component]
fn ProgressBar(playback: Playback) -> Element {
    let current_time = playback.current_time.read();
    let duration = playback.duration.read();

    let format_time = |s: f64| {
        let t = s.floor() as u64;
        format!("{:02}:{:02}", t / 60, t % 60)
    };

    rsx! {
        div { class: "w-full max-w-xl mx-auto",
            progress {
                class: "progress progress-primary w-full h-1.5",
                value: current_time.to_string(),
                max: duration.to_string(),
            }
            div { class: "flex justify-between text-xs opacity-60 mt-1",
                span { {format_time(*current_time)} }
                span { {format_time(*duration)} }
            }
        }
    }
}
