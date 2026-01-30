use dioxus::prelude::*;

use crate::components::icons::{MoonIcon, SunIcon};

#[component]
pub fn ThemeController() -> Element {
    let mut theme = use_signal(|| "light".to_string());

    use_effect(move || {
        let _ = document::eval(&format!(
            r#"
                document.documentElement.setAttribute('data-theme', '{}')
            "#,
            theme()
        ));
    });

    rsx! {
        label { class: "swap swap-rotate",
            input {
                r#type: "checkbox",
                class: "theme-controller",
                onclick: move |_| {
                    let new_theme = if theme() == "light" { "dark" } else { "light" };
                    theme.set(new_theme.to_string());
                },
            }
            SunIcon {}
            MoonIcon {}
        }
    }
}
