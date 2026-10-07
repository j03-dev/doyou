use dioxus::prelude::*;

use crate::common::components::alert::Alert;
use crate::common::components::icons::KeyIcon;
use crate::common::components::navbar::{NavBar, NavBarItem, NavBarPos};
use crate::common::components::text_input::TextInput;
use crate::common::context::use_settings;
use crate::core::utils::get_value_from;

#[component]
pub fn Setting() -> Element {
    let settings = use_settings();
    let nav  = use_navigator();
    let settings_error = settings.error;

    let submit_token = move |evt: Event<FormData>| {
        evt.prevent_default();
        let token = get_value_from(evt, "token").unwrap_or_default();
        settings.save_token(token);
    };

    rsx! {
        NavBar {
            NavBarItem { position: NavBarPos::Start,
                button {
                    class: "btn btn-ghost",
                    onclick: move |_| nav.go_back(),
                    "< Back"
                }
            }
            NavBarItem { position: NavBarPos::Center,
                p { class: "btn btn-ghost text-xl", "Settings" }
            }
            NavBarItem { position: NavBarPos::End,
                span {}
            }
        }
        div { class: "flex flex-col items-center justify-center px-6 pt-40",
            if let Some(alert_props) = settings_error() {
                Alert { ..alert_props }
            }
            p { class: "text-base-content/70 text-center mb-8 max-w-md",
                "Update your YouTube secret key to listen to music through the app."
            }
            form { class: "w-full max-w-md", onsubmit: submit_token,
                label { class: "label",
                    span { class: "label-text text-base-content/60 uppercase text-xs",
                        "Enter your secret key"
                    }
                }
                TextInput {
                    name: "token",
                    r#type: "password",
                    placeholder: "your-youtube-token",
                    value: settings.general.read().youtube_token.clone().unwrap_or_default(),
                    KeyIcon {}
                }
                button { class: "btn btn-primary w-full max-w-md mt-6", "Submit Token" }
            }
            p { class: "mt-4 text-sm text-base-content/60",
                span { "Don't have a token? " }
                a { class: "link link-primary", "Learn more" }
            }
        }
    }
}
