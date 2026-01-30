use dioxus::prelude::*;

use crate::components::icons::{LoadingSpinner, NextIcon, PauseIcon, PlayIcon, PrevIcon};
use crate::providers::Playback;

pub mod full_music_player;
pub mod mini_music_player;

#[component]
fn MusicController(mut playback: Playback) -> Element {
    rsx! {
        button {
            class: "btn btn-circle btn-ghost",
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
            button { class: "btn btn-ghost btn-primary btn-circle btn-xl", LoadingSpinner {} }
        }
        button {
            class: "btn btn-circle btn-ghost",
            onclick: move |_| playback.playback_controller(1),
            NextIcon {}
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
