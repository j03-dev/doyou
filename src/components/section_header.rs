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
            div { class: "flex items-center gap-2",
                if let Some(icon) = icon {
                    {icon}
                }
                h2 { class: "text-xs font-bold uppercase tracking-wider text-base-content/60",
                    {title}
                }
                if let Some(badge) = badge {
                    span { class: "badge badge-sm badge-ghost font-medium", {badge} }
                }
            }
            {children}
        }
    }
}
