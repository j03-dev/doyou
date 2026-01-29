use dioxus::prelude::*;

use crate::components::icons::{NextIcon, PauseIcon, PlayIcon, PrevIcon};
use crate::providers::Playback;

pub mod full_music_player;
pub mod mini_music_player;

#[component]
fn MusicController(mut playback: Playback) -> Element {
    rsx! {
        button {
            onclick: move |_| playback.playback_controller(-1),
            class: "btn btn-ghost btn-circle btn-md",
            PrevIcon {}
        }
        if !*playback.is_loading.read() {
            button {
                class: "btn btn-circle btn-primary btn-lg",
                onclick: move |_| playback.toggle_play(),
                if *playback.is_playing.read() {
                    PlayIcon {}
                } else {
                    PauseIcon {}
                }
            }
        } else {
            button { class: "btn btn-circle btn-primary btn-lg",
                span { class: "loading loading-spinner" }
            }
        }
        button {
            onclick: move |_| playback.playback_controller(1),
            class: "btn btn-ghost btn-circle btn-md",
            NextIcon {}
        }
    }
}

#[component]
fn ProgressBar(playback: Playback) -> Element {
    rsx! {
        div { class: "w-full max-w-xl mx-auto",
            progress {
                class: "progress progress-primary w-full h-1.5",
                value: playback.current_time.read().to_string(),
                max: playback.duration.read().to_string(),
            }
            div { class: "flex justify-between text-xs opacity-60 mt-1",
                span { "{format_time(*playback.current_time.read())}" }
                span { "{format_time(*playback.duration.read())}" }
            }
        }
    }
}

fn format_time(seconds: f64) -> String {
    let total = seconds.floor() as u64;
    format!("{:02}:{:02}", total / 60, total % 60)
}
