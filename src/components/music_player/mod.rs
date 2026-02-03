use dioxus::prelude::*;

use crate::components::icons::{LoadingSpinner, NextIcon, PauseIcon, PlayIcon, PrevIcon};
use crate::components::music_player::full_music_player::FullMusicPlayer;
use crate::components::music_player::mini_music_player::MiniMusicPlayer;
use crate::providers::Playback;

pub mod full_music_player;
pub mod mini_music_player;

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
fn MusicController(mut playback: Playback) -> Element {
    rsx! {
        div { class: "flex items-center justify-center gap-6 lg:gap-10",
            button {
                class: "btn btn-circle btn-ghost btn-secondary",
                onclick: move |_| playback.playback_controller(-1),
                PrevIcon {}
            }
            if !*playback.is_loading.read() {
                button {
                    class: "btn btn-circle btn-primary btn-xl",
                    onclick: move |_| playback.toggle_play(),
                    if *playback.is_playing.read() {
                        PlayIcon {}
                    } else {
                        PauseIcon {}
                    }
                }
            } else {
                button { class: "btn btn-circle btn-primary btn-xl", LoadingSpinner {} }
            }
            button {
                class: "btn btn-circle btn-ghost btn-secondary",
                onclick: move |_| playback.playback_controller(1),
                NextIcon {}
            }
        }
    }
}

#[component]
fn ProgressBar(playback: Playback) -> Element {
    let current_time = playback.current_time.read();
    let duration = playback.duration.read();

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

fn format_time(seconds: f64) -> String {
    let total = seconds.floor() as u64;
    format!("{:02}:{:02}", total / 60, total % 60)
}
