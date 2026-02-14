use dioxus::prelude::*;

use crate::common::components::button::{Button, IconButton};
use crate::common::components::icons::CloseIcon;
use crate::common::components::text_input::TextInput;
use crate::core::config::AppConfig;
use crate::core::utils::get_value_from;

#[component]
pub fn TokenForm(mut youtube_token: Signal<Option<String>>) -> Element {
    let mut status_msg = use_context::<Signal<Option<String>>>();

    let submit_token = move |evt: Event<FormData>| async move {
        evt.prevent_default();

        let token = get_value_from(evt, "token");
        if token.is_empty() {
            status_msg.set(Some("Please enter your youtube token".to_string()));
            return;
        }

        let config = AppConfig::new(token);
        if let Err(err) = config.save() {
            status_msg.set(Some(err.to_string()));
            return;
        }
        youtube_token.set(Some(config.youtube_token));
        document::eval("token_form.close()");
    };

    rsx! {
        dialog { id: "token_form", class: "modal",
            div { class: "modal-box",
                form { method: "dialog",
                    IconButton { class: "btn-sm absolute right-4 top-7", CloseIcon {} }
                }

                form { onsubmit: submit_token,
                    legend { class: "fieldset-legend", "youtube data api v3 key" }

                    label { class: "label", "Token" }
                    TextInput {
                        name: "token",
                        r#type: "text",
                        placeholder: "paste your api key here (e.g. AIzaSy...)",
                    }
                    Button { r#type: "submit", class: "w-full btn-primary mt-5", "Save" }
                }
            }
        }
    }
}
