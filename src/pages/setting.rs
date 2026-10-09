use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdKey;

use crate::components::alert::Alert;
use crate::components::form::get_value_from;
use crate::components::navbar::{NavBar, NavBarItem, NavBarPos};
use crate::components::text_input::TextInput;
use crate::context::use_settings;

#[component]
pub fn Setting() -> Element {
    let settings = use_settings();
    let nav = use_navigator();
    let settings_error = settings.error;

    let has_token = use_memo(move || settings.general.read().youtube_token.is_some());

    let submit_token = move |evt: Event<FormData>| {
        evt.prevent_default();
        let token = get_value_from(evt, "token").unwrap_or_default();
        settings.save_token(token);
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
        div { class: "mx-auto flex w-full max-w-md flex-col gap-4 px-4 py-6",
            if let Some(alert_props) = settings_error() {
                Alert { ..alert_props }
            }
            div { class: "card card-border bg-base-100",
                div { class: "card-body",
                    div { class: "flex items-center justify-between",
                        h2 { class: "card-title", "YouTube API" }
                        if has_token() {
                            span { class: "badge badge-soft badge-success", "Configured" }
                        } else {
                            span { class: "badge badge-soft badge-warning", "Not configured" }
                        }
                    }
                    p { class: "text-sm text-base-content/70",
                        "Update your YouTube secret key to listen to music through the app."
                    }
                    form { onsubmit: submit_token,
                        fieldset { class: "fieldset",
                            legend { class: "fieldset-legend", "Enter your secret key" }
                            TextInput {
                                name: "token",
                                r#type: "password",
                                placeholder: "your-youtube-token",
                                value: settings.general.read().youtube_token.clone().unwrap_or_default(),
                                Icon { icon: LdKey }
                            }
                        }
                        button { class: "btn btn-primary mt-4 w-full", "Submit Token" }
                    }
                    p { class: "mt-2 text-sm text-base-content/60",
                        span { "Don't have a token? " }
                        a { class: "link link-primary", "Learn more" }
                    }
                }
            }
            div { class: "card card-border bg-base-100",
                div { class: "card-body",
                    h2 { class: "card-title", "About" }
                    div { class: "flex items-center gap-2",
                        p { class: "text-lg font-semibold", "{env!(\"CARGO_PKG_NAME\")}" }
                        span { class: "badge badge-ghost", {format!("v{}", env!("CARGO_PKG_VERSION"))} }
                    }
                    p { class: "text-sm text-base-content/70",
                        "A cross-platform desktop app to listen to music from YouTube."
                    }
                }
            }
        }
    }
}
