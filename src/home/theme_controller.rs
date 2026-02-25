use dioxus::prelude::*;

use crate::common::{
    components::icons::BurgerIcon,
    context::{use_alert, use_settings},
};

#[component]
pub fn ThemeController() -> Element {
    let settings = use_settings();
    let mut alert = use_alert();

    let themes = &[
        "Lofi",
        "Black",
        "Night",
        "Halloween",
        "Lemonade",
        "Forest",
        "Dracula",
    ];

    use_effect(move || {
        if let Some(err) = settings.error.read().as_ref() {
            alert.message.set(Some(err.clone()));
        }
    });

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
                for theme in themes {
                    ThemeItem {
                        name: theme,
                        callback: move |theme| {
                            settings.save_theme(theme);
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn ThemeItem(name: &'static str, callback: Callback<String>) -> Element {
    rsx! {
        li {
            input {
                r#type: "radio",
                name: "theme-dropdown",
                class: "theme-controller w-full btn btn-sm btn-block btn-ghost justify-start",
                aria_label: name,
                value: name.to_lowercase(),
                onclick: move |_| callback.call(name.to_lowercase()),
            }
        }
    }
}
