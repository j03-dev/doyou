use dioxus::prelude::*;

#[component]
pub fn EmptyState(
    title: String,
    #[props(default)] message: Option<String>,
    #[props(default)] icon: Option<Element>,
    #[props(default)] children: Element,
) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center gap-3 px-6 py-16 text-center",
            if let Some(icon) = icon {
                div { class: "flex size-16 items-center justify-center rounded-full bg-base-200 text-base-content/40",
                    {icon}
                }
            }
            h3 { class: "text-lg font-semibold", {title} }
            if let Some(message) = message {
                p { class: "max-w-xs text-sm text-base-content/60", {message} }
            }
            {children}
        }
    }
}
