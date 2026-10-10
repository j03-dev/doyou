use std::time::Instant;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{
    LdActivity, LdArrowLeft, LdCast, LdCheck, LdChevronRight, LdCpu, LdExternalLink, LdEye,
    LdEyeOff, LdInfo, LdKey, LdPalette, LdRefreshCw, LdVolume2,
};

use crate::components::alert::Alert;
use crate::components::form::get_value_from;
use crate::components::navbar::{NavBar, NavBarItem, NavBarPos};
use crate::components::text_input::TextInput;
use crate::context::{use_playback, use_settings};
use crate::repository;

#[cfg(target_os = "android")]
const AUDIO_ENGINE: &str = "ExoPlayer + FFmpeg";
#[cfg(not(target_os = "android"))]
const AUDIO_ENGINE: &str = "HTML5 Audio";

#[derive(Clone, PartialEq)]
enum PingStatus {
    Idle,
    Testing,
    Success(u128),
    Error(String),
}

#[component]
pub fn Setting() -> Element {
    let settings = use_settings();
    let playback = use_playback();
    let nav = use_navigator();
    let settings_error = settings.error;

    let has_token = use_memo(move || settings.general.read().youtube_token.is_some());
    let mut show_form = use_signal(|| false);
    let mut show_plain_key = use_signal(|| false);
    let mut ping_status = use_signal(|| PingStatus::Idle);

    let token = settings
        .general
        .read()
        .youtube_token
        .clone()
        .unwrap_or_default();
    let masked = mask_key(&token);
    let current_theme = settings.general.read().theme.to_lowercase();

    let submit_token = move |evt: Event<FormData>| {
        evt.prevent_default();
        let new_token = get_value_from(evt, "token").unwrap_or_default();
        settings.save_token(new_token.clone());
        if !new_token.is_empty() {
            show_form.set(false);
        }
    };

    let test_connection = move |_| {
        let Some(current_token) = settings.general.read().youtube_token.clone() else {
            ping_status.set(PingStatus::Error("Configure an API key first".to_string()));
            return;
        };
        let mut status = ping_status;
        status.set(PingStatus::Testing);
        spawn(async move {
            let start = Instant::now();
            let result = repository::youtube::home(&current_token).await;
            let ms = start.elapsed().as_millis();
            match result {
                Ok(_) => status.set(PingStatus::Success(ms)),
                Err(err) => status.set(PingStatus::Error(err)),
            }
        });
    };

    let themes = &[
        ("Lofi", "bg-neutral", "bg-base-300"),
        ("Black", "bg-zinc-800", "bg-zinc-950"),
        ("Nord", "bg-blue-400", "bg-slate-700"),
        ("Pastel", "bg-pink-300", "bg-purple-200"),
        ("Slik", "bg-emerald-400", "bg-zinc-800"),
        ("Sunset", "bg-orange-400", "bg-indigo-900"),
    ];

    rsx! {
        NavBar {
            NavBarItem { position: NavBarPos::Start,
                button {
                    class: "btn btn-ghost btn-circle",
                    "aria-label": "Back",
                    onclick: move |_| nav.go_back(),
                    Icon { icon: LdArrowLeft }
                }
            }
            NavBarItem { position: NavBarPos::Center,
                p { class: "btn btn-ghost text-xl", "Settings" }
            }
            NavBarItem { position: NavBarPos::End,
                span {}
            }
        }

        div {
            class: format!(
                "mx-auto w-full max-w-2xl px-4 py-4 sm:py-6 flex flex-col gap-6 {}",
                if playback.playing.read().is_some() { "pb-48" } else { "pb-28" }
            ),
            if let Some(alert_props) = settings_error() {
                Alert { ..alert_props }
            }

            // Hero Brand Summary
            div { class: "flex items-center gap-3.5 p-4 rounded-2xl bg-base-100 border border-base-content/10 shadow-sm",
                div { class: "flex size-12 shrink-0 items-center justify-center rounded-xl bg-primary/10 text-primary",
                    Icon { icon: LdVolume2, class: "size-6" }
                }
                div { class: "min-w-0 flex-1",
                    div { class: "flex items-center gap-2",
                        h1 { class: "text-base sm:text-lg font-bold leading-tight", "DoYou Music" }
                        span { class: "badge badge-neutral badge-xs font-mono font-medium",
                            {format!("v{}", env!("CARGO_PKG_VERSION"))}
                        }
                    }
                    p { class: "text-xs text-base-content/60 truncate mt-0.5",
                        "Cross-platform YouTube streaming desktop & mobile client"
                    }
                }
            }

            // Section 1: YouTube Integration & API
            div { class: "flex flex-col gap-2.5",
                div { class: "flex items-center justify-between px-1",
                    div { class: "flex items-center gap-2",
                        Icon { icon: LdKey, class: "size-4 text-primary" }
                        h2 { class: "text-xs font-bold uppercase tracking-wider text-base-content/60",
                            "YouTube Integration"
                        }
                    }
                    if has_token() {
                        span { class: "badge badge-sm badge-success gap-1 text-xs font-medium",
                            span { class: "size-1.5 rounded-full bg-current animate-pulse" }
                            "Connected"
                        }
                    } else {
                        span { class: "badge badge-sm badge-warning text-xs font-medium",
                            "Key Required"
                        }
                    }
                }

                div { class: "card bg-base-100 border border-base-content/10 shadow-sm rounded-2xl overflow-hidden",
                    ul { class: "list divide-y divide-base-content/5",
                        li { class: "list-row flex items-center justify-between p-3.5 sm:p-4 gap-3",
                            div { class: "flex size-9 shrink-0 items-center justify-center rounded-xl bg-base-200 text-primary",
                                Icon { icon: LdCast, class: "size-5" }
                            }
                            div { class: "min-w-0 flex-1",
                                div { class: "text-sm font-semibold", "YouTube Data API v3" }
                                div { class: "text-xs text-base-content/60 truncate",
                                    "Powers feed recommendations, search & metadata"
                                }
                            }
                            if has_token() {
                                span { class: "badge badge-soft badge-success text-xs shrink-0 font-medium", "Active" }
                            } else {
                                span { class: "badge badge-soft badge-error text-xs shrink-0 font-medium", "Inactive" }
                            }
                        }

                        li { class: "list-row flex items-center justify-between p-3.5 sm:p-4 gap-3",
                            div { class: "flex size-9 shrink-0 items-center justify-center rounded-xl bg-base-200 text-primary",
                                Icon { icon: LdKey, class: "size-5" }
                            }
                            div { class: "min-w-0 flex-1",
                                div { class: "text-sm font-semibold", "API Key" }
                                div { class: "text-xs font-mono text-base-content/60 truncate",
                                    {masked}
                                }
                            }
                            button {
                                class: "btn btn-sm btn-outline shrink-0",
                                r#type: "button",
                                onclick: move |_| show_form.set(!show_form()),
                                if show_form() { "Cancel" } else { "Edit Key" }
                            }
                        }

                        li { class: "list-row flex items-center justify-between p-3.5 sm:p-4 gap-3",
                            div { class: "flex size-9 shrink-0 items-center justify-center rounded-xl bg-base-200 text-primary",
                                Icon { icon: LdActivity, class: "size-5" }
                            }
                            div { class: "min-w-0 flex-1",
                                div { class: "text-sm font-semibold", "Latency Diagnostic" }
                                div { class: "text-xs text-base-content/60",
                                    match ping_status() {
                                        PingStatus::Idle => rsx! { span { "Check response latency with YouTube servers" } },
                                        PingStatus::Testing => rsx! { span { class: "loading loading-spinner loading-xs text-primary" } },
                                        PingStatus::Success(ms) => rsx! {
                                            span { class: "text-success font-medium font-mono",
                                                "{ms}ms roundtrip"
                                            }
                                        },
                                        PingStatus::Error(ref err) => rsx! {
                                            span { class: "text-error font-medium truncate",
                                                "{err}"
                                            }
                                        },
                                    }
                                }
                            }
                            button {
                                class: "btn btn-sm btn-primary shrink-0 gap-1.5",
                                r#type: "button",
                                disabled: ping_status() == PingStatus::Testing,
                                onclick: test_connection,
                                if ping_status() == PingStatus::Testing {
                                    span { class: "loading loading-spinner loading-xs" }
                                } else {
                                    Icon { icon: LdRefreshCw, class: "size-3.5" }
                                }
                                "Ping"
                            }
                        }
                    }

                    if show_form() {
                        form {
                            class: "border-t border-base-content/10 bg-base-200/30 p-4 sm:p-5 flex flex-col gap-3",
                            onsubmit: submit_token,
                            div { class: "flex items-center justify-between",
                                span { class: "text-sm font-semibold", "Enter YouTube API Key" }
                                a {
                                    class: "link link-primary text-xs flex items-center gap-1",
                                    href: "https://console.cloud.google.com/apis/credentials",
                                    target: "_blank",
                                    "Get API key"
                                    Icon { icon: LdExternalLink, class: "size-3" }
                                }
                            }
                            div { class: "relative w-full",
                                TextInput {
                                    name: "token",
                                    r#type: if show_plain_key() { "text" } else { "password" },
                                    placeholder: "AIzaSy...",
                                    value: token.clone(),
                                    Icon { icon: LdKey }
                                }
                                button {
                                    class: "btn btn-ghost btn-xs btn-circle absolute right-3 top-1/2 -translate-y-1/2 opacity-70 hover:opacity-100",
                                    r#type: "button",
                                    onclick: move |_| show_plain_key.set(!show_plain_key()),
                                    if show_plain_key() {
                                        Icon { icon: LdEyeOff, class: "size-4" }
                                    } else {
                                        Icon { icon: LdEye, class: "size-4" }
                                    }
                                }
                            }
                            div { class: "flex items-center justify-end gap-2 pt-1",
                                button {
                                    class: "btn btn-sm btn-ghost",
                                    r#type: "button",
                                    onclick: move |_| show_form.set(false),
                                    "Cancel"
                                }
                                button {
                                    class: "btn btn-sm btn-primary",
                                    r#type: "submit",
                                    "Save Key"
                                }
                            }
                        }
                    }
                }
            }

            // Section 2: Appearance & Theme
            div { class: "flex flex-col gap-2.5",
                div { class: "flex items-center gap-2 px-1",
                    Icon { icon: LdPalette, class: "size-4 text-secondary" }
                    h2 { class: "text-xs font-bold uppercase tracking-wider text-base-content/60",
                        "Appearance & Theme"
                    }
                }

                div { class: "card bg-base-100 border border-base-content/10 shadow-sm rounded-2xl p-3 sm:p-4",
                    div { class: "grid grid-cols-2 sm:grid-cols-3 gap-2.5",
                        for (name, col1, col2) in themes {
                            {
                                let is_selected = current_theme == name.to_lowercase();
                                let theme_str = name.to_lowercase();
                                rsx! {
                                    button {
                                        r#type: "button",
                                        class: format!(
                                            "flex items-center justify-between p-3 rounded-xl border transition-all text-left {}",
                                            if is_selected {
                                                "border-primary bg-primary/10 ring-2 ring-primary text-primary font-semibold shadow-sm"
                                            } else {
                                                "border-base-content/10 bg-base-200/40 hover:bg-base-200/80 hover:border-base-content/20 text-base-content"
                                            }
                                        ),
                                        onclick: move |_| settings.save_theme(theme_str.clone()),
                                        div { class: "flex items-center gap-2.5 min-w-0",
                                            div { class: "flex items-center shrink-0",
                                                div { class: format!("size-3 rounded-full border border-base-content/10 {}", col1) }
                                                div { class: format!("size-3 rounded-full border border-base-content/10 -ml-1 {}", col2) }
                                            }
                                            span { class: "text-sm truncate", "{name}" }
                                        }
                                        if is_selected {
                                            Icon { icon: LdCheck, class: "size-4 text-primary shrink-0" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Section 3: Audio & Engine
            div { class: "flex flex-col gap-2.5",
                div { class: "flex items-center gap-2 px-1",
                    Icon { icon: LdCpu, class: "size-4 text-accent" }
                    h2 { class: "text-xs font-bold uppercase tracking-wider text-base-content/60",
                        "Audio Engine & Pipeline"
                    }
                }

                div { class: "card bg-base-100 border border-base-content/10 shadow-sm rounded-2xl overflow-hidden",
                    ul { class: "list divide-y divide-base-content/5",
                        li { class: "list-row flex items-center justify-between p-3.5 sm:p-4 gap-3",
                            div { class: "flex size-9 shrink-0 items-center justify-center rounded-xl bg-base-200 text-accent",
                                Icon { icon: LdVolume2, class: "size-5" }
                            }
                            div { class: "min-w-0 flex-1",
                                div { class: "text-sm font-semibold", "Playback Engine" }
                                div { class: "text-xs text-base-content/60 truncate",
                                    "Underlying audio decode pipeline"
                                }
                            }
                            span { class: "badge badge-neutral text-xs font-medium font-mono shrink-0",
                                {AUDIO_ENGINE}
                            }
                        }

                        li { class: "list-row flex items-center justify-between p-3.5 sm:p-4 gap-3",
                            div { class: "flex size-9 shrink-0 items-center justify-center rounded-xl bg-base-200 text-accent",
                                Icon { icon: LdCpu, class: "size-5" }
                            }
                            div { class: "min-w-0 flex-1",
                                div { class: "text-sm font-semibold", "Audio Extraction" }
                                div { class: "text-xs text-base-content/60 truncate",
                                    "Direct Opus / AAC stream extractor"
                                }
                            }
                            span { class: "badge badge-soft badge-success text-xs font-medium shrink-0",
                                "Optimized"
                            }
                        }
                    }
                }
            }

            // Section 4: About & Links
            div { class: "flex flex-col gap-2.5",
                div { class: "flex items-center gap-2 px-1",
                    Icon { icon: LdInfo, class: "size-4 text-info" }
                    h2 { class: "text-xs font-bold uppercase tracking-wider text-base-content/60",
                        "About & Resources"
                    }
                }

                div { class: "card bg-base-100 border border-base-content/10 shadow-sm rounded-2xl overflow-hidden",
                    ul { class: "list divide-y divide-base-content/5",
                        li { class: "list-row p-3.5 sm:p-4",
                            a {
                                class: "flex w-full items-center justify-between gap-3 group",
                                href: "https://github.com/j03-dev/doyou",
                                target: "_blank",
                                div { class: "flex size-9 shrink-0 items-center justify-center rounded-xl bg-base-200 text-info transition-colors group-hover:bg-primary/10 group-hover:text-primary",
                                    Icon { icon: LdExternalLink, class: "size-5" }
                                }
                                div { class: "min-w-0 flex-1",
                                    div { class: "text-sm font-semibold group-hover:text-primary transition-colors", "Source Code & Licenses" }
                                    div { class: "text-xs text-base-content/60 truncate",
                                        "github.com/j03-dev/doyou"
                                    }
                                }
                                Icon { icon: LdChevronRight, class: "size-4 text-base-content/40 group-hover:text-primary group-hover:translate-x-0.5 transition-all shrink-0" }
                            }
                        }

                        li { class: "list-row p-3.5 sm:p-4",
                            a {
                                class: "flex w-full items-center justify-between gap-3 group",
                                href: "https://github.com/j03-dev/doyou/issues",
                                target: "_blank",
                                div { class: "flex size-9 shrink-0 items-center justify-center rounded-xl bg-base-200 text-info transition-colors group-hover:bg-error/10 group-hover:text-error",
                                    Icon { icon: LdInfo, class: "size-5" }
                                }
                                div { class: "min-w-0 flex-1",
                                    div { class: "text-sm font-semibold group-hover:text-error transition-colors", "Report an Issue" }
                                    div { class: "text-xs text-base-content/60 truncate",
                                        "Submit bug reports or feature requests"
                                    }
                                }
                                Icon { icon: LdChevronRight, class: "size-4 text-base-content/40 group-hover:text-error group-hover:translate-x-0.5 transition-all shrink-0" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn mask_key(key: &str) -> String {
    if key.is_empty() {
        "Not configured".to_string()
    } else if key.len() <= 10 {
        "••••••••••••".to_string()
    } else {
        let (head, _) = key.split_at(6);
        let (_, tail) = key.split_at(key.len().saturating_sub(4));
        format!("{head}••••••••{tail}")
    }
}
