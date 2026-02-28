use dioxus::prelude::*;

use crate::common::components::button::{Button, IconButton};
use crate::common::components::icons::CloseIcon;
use crate::common::components::text_input::TextInput;
use crate::common::context::use_settings;
use crate::core::utils::get_value_from;

#[component]
pub fn TokenForm() -> Element {
    let settings = use_settings();

    let submit_token = move |evt: Event<FormData>| {
        evt.prevent_default();
        let token = get_value_from(evt, "token");
        settings.save_token(token.unwrap());
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
