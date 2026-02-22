use dioxus::prelude::*;

use crate::common::components::button::{Button, IconButton};
use crate::common::components::icons::CloseIcon;
use crate::common::components::text_input::TextInput;
use crate::core::db;
use crate::core::utils::get_value_from;

#[component]
pub fn TokenForm(mut youtube_token: Signal<Option<String>>) -> Element {
    let mut status_msg = use_context::<Signal<Option<String>>>();

    let submit_token = move |evt: Event<FormData>| async move {
        evt.prevent_default();

        let token = get_value_from(evt, "token");
        if token.is_none() {
            dbg!(token);
            status_msg.set(Some("Please enter your youtube token".to_string()));
            return;
        }

        let token = token.unwrap();

        if let Err(err) = db::save_token(&token).await {
            status_msg.set(Some(err.to_string()));
            return;
        }
        youtube_token.set(Some(token));
        document::eval("token_form.close()");
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
