use std::time::Instant;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{
    LdActivity, LdCast, LdChevronRight, LdHeadphones, LdInfo, LdKey, LdRefreshCw,
};

use crate::components::alert::Alert;
use crate::components::form::get_value_from;
use crate::components::navbar::{NavBar, NavBarItem, NavBarPos};
use crate::components::text_input::TextInput;
use crate::context::use_settings;
use crate::repository;

#[cfg(target_os = "android")]
const AUDIO_ENGINE: &str = "ExoPlayer + FFmpeg";
#[cfg(not(target_os = "android"))]
const AUDIO_ENGINE: &str = "HTML5 Audio";

#[component]
pub fn Setting() -> Element {
    let settings = use_settings();
    let nav = use_navigator();
    let settings_error = settings.error;

    let has_token = use_memo(move || settings.general.read().youtube_token.is_some());
    let mut show_form = use_signal(|| false);
    let ping_testing = use_signal(|| false);
    let mut ping_label = use_signal(|| "Latency: —".to_string());

    let token = settings
        .general
        .read()
        .youtube_token
        .clone()
        .unwrap_or_default();
    let masked = mask_key(&token);

    let submit_token = move |evt: Event<FormData>| {
        evt.prevent_default();
        let token = get_value_from(evt, "token").unwrap_or_default();
        settings.save_token(token.clone());
        if !token.is_empty() {
            show_form.set(false);
        }
    };

    let test_connection = move |_| {
        let Some(token) = settings.general.read().youtube_token.clone() else {
            ping_label.set("Configure an API key first".to_string());
            return;
        };
        let mut testing = ping_testing;
        let mut label = ping_label;
        testing.set(true);
        label.set("Testing connection...".to_string());
        spawn(async move {
            let start = Instant::now();
            let result = repository::youtube::home(&token).await;
            let ms = start.elapsed().as_millis();
            testing.set(false);
            match result {
                Ok(_) => label.set(format!("Connected · {}ms", ms)),
                Err(err) => label.set(format!("Failed · {}", err)),
            }
        });
    };

    rsx! {
        NavBar {
            NavBarItem { position: NavBarPos::Start,
                button { class: "btn btn-ghost", onclick: move |_| nav.go_back(), "< Back" }
            }
            NavBarItem { position: NavBarPos::Center,
                p { class: "btn btn-ghost text-xl", "Settings" }
            }
            NavBarItem { position: NavBarPos::End,
                span {}
            }
        }
        div { class: "mx-auto w-full max-w-3xl px-4 py-6 lg:px-8",
            if let Some(alert_props) = settings_error() {
                Alert { ..alert_props }
            }
            div { class: "mt-6 grid gap-6 lg:grid-cols-2",
                div { class: "flex flex-col gap-2",
                    SectionLabel { "YouTube Integration & API" }
                    div { class: "card card-border bg-base-100",
                        ul { class: "list",
                            ListRow {
                                RowIcon {
                                    Icon { icon: LdCast, class: "size-5" }
                                }
                                RowText {
                                    title: "YouTube API Connection",
                                    subtitle: "v3 Data API Service".to_string(),
                                }
                                if has_token() {
                                    span { class: "badge badge-soft badge-success gap-1.5",
                                        span { class: "size-1.5 rounded-full bg-current animate-pulse" }
                                        "Connected"
                                    }
                                } else {
                                    span { class: "badge badge-soft badge-warning", "Not connected" }
                                }
                            }
                            ListRow {
                                RowIcon {
                                    Icon { icon: LdKey, class: "size-5" }
                                }
                                RowText {
                                    title: "API Key",
                                    subtitle: masked.clone(),
                                }
                                button {
                                    class: "btn btn-outline btn-sm",
                                    r#type: "button",
                                    onclick: move |_| show_form.set(true),
                                    "Edit"
                                }
                            }
                            ListRow {
                                RowIcon {
                                    Icon { icon: LdActivity, class: "size-5" }
                                }
                                RowText {
                                    title: "Diagnostic Ping",
                                    subtitle: ping_label(),
                                }
                                button {
                                    class: "btn btn-primary btn-sm gap-1.5",
                                    r#type: "button",
                                    onclick: test_connection,
                                    if ping_testing() {
                                        span { class: "loading loading-spinner loading-xs" }
                                    } else {
                                        Icon {
                                            icon: LdRefreshCw,
                                            class: "size-4",
                                        }
                                    }
                                    "Test"
                                }
                            }
                        }
                        if show_form() {
                            form {
                                class: "border-t border-base-content/5 p-4",
                                onsubmit: submit_token,
                                fieldset { class: "fieldset",
                                    legend { class: "fieldset-legend", "Enter your secret key" }
                                    TextInput {
                                        name: "token",
                                        r#type: "password",
                                        placeholder: "your-youtube-token",
                                        value: token,
                                        Icon { icon: LdKey }
                                    }
                                }
                                div { class: "mt-3 flex gap-3",
                                    button {
                                        class: "btn btn-primary w-full",
                                        r#type: "submit",
                                        "Save"
                                    }
                                    button {
                                        class: "btn w-full",
                                        r#type: "button",
                                        onclick: move |_| show_form.set(false),
                                        "Cancel"
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "flex flex-col gap-2",
                    SectionLabel { "About & Engine" }
                    div { class: "card card-border bg-base-100",
                        ul { class: "list",
                            ListRow {
                                div { class: "flex w-full items-center justify-between gap-4",
                                    span { class: "text-sm font-medium", "App Version" }
                                    span { class: "font-mono text-sm text-base-content/60",
                                        {format!("v{}", env!("CARGO_PKG_VERSION"))}
                                    }
                                }
                            }
                            ListRow {
                                div { class: "flex w-full items-center justify-between gap-4",
                                    span { class: "text-sm font-medium", "Audio Engine" }
                                    span { class: "max-w-[55%] truncate text-right text-sm font-medium text-primary",
                                        {AUDIO_ENGINE}
                                    }
                                }
                            }
                            ListRow {
                                a {
                                    class: "flex w-full items-center justify-between gap-4",
                                    href: "https://github.com/j03-dev/doyou",
                                    span { class: "text-sm font-medium", "Open Source Licenses" }
                                    Icon {
                                        icon: LdChevronRight,
                                        class: "size-4 text-base-content/40",
                                    }
                                }
                            }
                            ListRow {
                                a {
                                    class: "flex w-full items-center justify-between gap-4",
                                    href: "https://github.com/j03-dev/doyou/issues",
                                    span { class: "text-sm font-medium", "Report an Issue & Logs" }
                                    Icon {
                                        icon: LdChevronRight,
                                        class: "size-4 text-base-content/40",
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SectionLabel(children: Element) -> Element {
    rsx! {
        div { class: "px-1 pt-1 text-xs font-semibold uppercase tracking-wider text-base-content/60",
            {children}
        }
    }
}

#[component]
fn ListRow(children: Element) -> Element {
    rsx! {
        li { class: "list-row transition-colors hover:bg-base-200/40", {children} }
    }
}

#[component]
fn RowIcon(children: Element) -> Element {
    rsx! {
        div { class: "flex size-8 shrink-0 items-center justify-center self-center rounded-lg bg-base-200 text-primary",
            {children}
        }
    }
}

#[component]
fn RowText(title: &'static str, subtitle: String) -> Element {
    rsx! {
        div { class: "min-w-0",
            div { class: "truncate text-sm font-medium", {title} }
            div { class: "truncate text-xs text-base-content/60", {subtitle} }
        }
    }
}

fn mask_key(key: &str) -> String {
    let glyphs = "••••••••••".to_string();
    if key.is_empty() {
        "Not set".to_string()
    } else if key.len() <= 8 {
        glyphs
    } else {
        let (head, _) = key.split_at(7);
        format!("{head}{glyphs}")
    }
}
