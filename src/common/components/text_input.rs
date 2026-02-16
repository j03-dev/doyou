use dioxus::prelude::*;

#[component]
pub fn TextInput(
    #[props(default)] name: &'static str,
    #[props(default)] r#type: &'static str,
    #[props(default)] placeholder: &'static str,
    #[props[default]] children: Element,
) -> Element {
    rsx! {
        div { class: "flex flex-row jusify-center w-full gap-2",
            label { class: "input input-md input-primary w-full",
                {children}
                input {
                    name,
                    r#type,
                    class: "grow",
                    placeholder,
                }
            }
        }
    }
}
