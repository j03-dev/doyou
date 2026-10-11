use dioxus::prelude::*;

use crate::components::cover::CoverCollage;

#[component]
pub fn CollectionHero(
    name: String,
    #[props(default)] eyebrow: Option<String>,
    #[props(default)] thumbnails: Vec<String>,
    #[props(default)] count: usize,
    #[props(default)] accent: Option<String>,
    #[props(default)] children: Element,
) -> Element {
    let accent = accent.unwrap_or_else(|| "text-primary".to_string());

    rsx! {
        section { class: "relative",
            div { class: "pointer-events-none absolute -top-10 left-4 size-40 rounded-full bg-primary/20 blur-3xl" }
            div { class: "relative flex flex-col items-center gap-5 sm:flex-row sm:items-end",
                CoverCollage {
                    thumbnails,
                    class: "w-40 rounded-2xl shadow-xl sm:w-44",
                }
                div { class: "min-w-0 text-center sm:pb-1 sm:text-left",
                    if let Some(eyebrow) = eyebrow {
                        div { class: "mb-1.5 flex items-center justify-center gap-1.5 sm:justify-start",
                            span { class: "size-1.5 rounded-full bg-current {accent}" }
                            p { class: "text-xs font-bold uppercase tracking-wider {accent}",
                                {eyebrow}
                            }
                        }
                    }
                    h1 {
                        class: "truncate text-3xl font-bold leading-tight sm:text-4xl",
                        title: name.clone(),
                        {name.clone()}
                    }
                    p { class: "mt-1 text-sm text-base-content/60",
                        "{count} "
                        if count == 1 {
                            "song"
                        } else {
                            "songs"
                        }
                    }
                }
            }
            div { class: "relative mt-5 flex flex-wrap items-center justify-center gap-3 sm:justify-start",
                {children}
            }
        }
    }
}
