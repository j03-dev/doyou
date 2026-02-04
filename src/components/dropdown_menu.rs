use dioxus::prelude::*;

use crate::components::icons::BurgerIcon;

#[component]
pub fn DropDownMenu() -> Element {
    rsx! {
        div { class: "dropdown",
            div {
                tabindex: 0,
                role: "button",
                class: "btn btn-ghost btn-circle",
                BurgerIcon {}
            }
            ul {
                tabindex: -1,
                class: "dropdown-content bg-base-300 rounded-box z-1 w-52 p-2 shadow-2xl",
                li {
                    ThemeControllerDropDown { name: "Nord" }
                }
                li {
                    ThemeControllerDropDown { name: "Black" }
                }
            }
        }
    }
}

#[component]
fn ThemeControllerDropDown(name: &'static str) -> Element {
    rsx! {
        input {
            r#type: "radio",
            name: "theme-dropdown",
            class: "theme-controller w-full btn btn-sm btn-block btn-ghost justify-start",
            aria_label: name,
            value: name.to_lowercase(),
        }
    }
}
