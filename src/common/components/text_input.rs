use dioxus::prelude::*;

#[component]
pub fn TextInput(
    #[props(default)] name: &'static str,
    #[props(default)] r#type: &'static str,
    #[props(default)] placeholder: &'static str,
    #[props[default]] on_submit: EventHandler<Event<FormData>>,
    #[props[default]] children: Element,
) -> Element {
    rsx! {
        form {
            class: "flex flex-row jusify-center w-full gap-2",
            onsubmit: on_submit,
            label { class: "input input-md input-primary",
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
