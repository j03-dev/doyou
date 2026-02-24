use dioxus::prelude::*;

use crate::common::components::{button::Button, text_input::TextInput};

#[component]
pub fn Setting() -> Element {
    rsx! {
        div {
            class: "w-full flex flex-col justify-center items-center",
            div { class: "bg-base-100 rounded-xl max-w-md",
                form {
                    TextInput { placeholder: "add you google token here!" }
                    Button { class: "btn-primary", "Save" }
                }
            }
        }
    }
}
