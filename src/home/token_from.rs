use dioxus::prelude::*;

use crate::common::components::button::{Button, IconButton};
use crate::common::components::icons::CloseIcon;
use crate::common::components::text_input::TextInput;
use crate::common::context::use_alert;
use crate::core::db;
use crate::core::utils::get_value_from;

#[component]
pub fn TokenForm(mut youtube_token: Signal<Option<String>>) -> Element {
    let alert = use_alert();
    let alert_message = alert.message.clone();

    let submit_token = move |evt: Event<FormData>| {
        let mut msg = alert_message.clone();
        
        evt.prevent_default();

        let token = get_value_from(evt, "token");
        if token.is_none() {
            msg.set(Some("Please enter your youtube token".to_string()));
            return;
        }

        let token = token.unwrap();
        let mut token_signal = youtube_token.clone();
        
        spawn(async move {
            if let Err(err) = db::save_token(&token).await {
                msg.set(Some(err.to_string()));
            } else {
                token_signal.set(Some(token));
                document::eval("token_form.close()");
            }
        });
    };

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
