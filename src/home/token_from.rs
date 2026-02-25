use dioxus::prelude::*;

use crate::common::components::button::{Button, IconButton};
use crate::common::components::icons::CloseIcon;
use crate::common::components::text_input::TextInput;
use crate::common::context::{use_alert, use_settings};
use crate::core::utils::get_value_from;

#[component]
pub fn TokenForm() -> Element {
    let settings = use_settings();
    let mut alert = use_alert();
    let mut show_success = use_signal(|| false);

    use_effect(move || {
        if let Some(err_msg) = settings.error.read().as_ref() {
            alert.message.set(Some(err_msg.clone()));
        }
    });

    use_effect(move || {
        if show_success() {
            document::eval("token_form.close()");
            show_success.set(false);
        }
    });

    let submit_token = move |evt: Event<FormData>| {
        evt.prevent_default();

        let token = get_value_from(evt, "token");
        if token.is_none() {
            alert
                .message
                .set(Some("Please enter your youtube token".to_string()));
            return;
        }

        settings.save_token(token.unwrap());
    };

    use_effect(move || {
        if settings.general.read().youtube_token.is_some() {
            show_success.set(true);
        }
    });

    rsx! {
        dialog { id: "token_form", class: "modal",
            div { class: "modal-box w-96",
                form { method: "dialog",
                    IconButton { class: "btn-sm absolute right-4 top-7", CloseIcon {} }
                }
                br {}
                form { onsubmit: submit_token,
                    legend { class: "fieldset-legend", "Youtube Token" }
                    TextInput {
                        name: "token",
                        r#type: "password",
                        placeholder: "paste your api key here (e.g. AIzaSy...)",
                    }
                    Button { r#type: "submit", class: "w-full btn-primary mt-5", "Save" }
                }
            }
        }
    }
}
