use dioxus::prelude::*;

#[component]
pub fn SectionHeader(
    title: String,
    #[props(default)] icon: Option<Element>,
    #[props(default)] badge: Option<String>,
    #[props(default)] children: Element,
) -> Element {
    rsx! {
        div { class: "flex items-center justify-between gap-3 px-1",
            div { class: "flex items-center gap-2.5",
                if let Some(icon) = icon {
                    div { class: "flex size-8 shrink-0 items-center justify-center rounded-lg bg-base-200 text-base-content/70",
                        {icon}
                    }
                }
                h2 { class: "text-sm font-bold tracking-tight text-base-content",
                    {title}
                }
                if let Some(badge) = badge {
                    span { class: "badge badge-sm badge-ghost font-semibold", {badge} }
                }
            }
            {children}
        }
    }
}
