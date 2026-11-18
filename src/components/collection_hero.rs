use dioxus::prelude::*;

use crate::components::cover::CoverCollage;

#[component]
pub fn CollectionHero(
    name: String,
    #[props(default)] eyebrow: Option<String>,
    #[props(default)] subtitle: Option<String>,
    #[props(default)] thumbnails: Vec<String>,
    #[props(default)] count: usize,
    #[props(default)] accent: Option<String>,
    #[props(default)] children: Element,
) -> Element {
    let accent = accent.unwrap_or_else(|| "text-primary".to_string());

    rsx! {
        section { class: "relative",
            div { class: "pointer-events-none absolute -top-10 left-1/2 size-48 -translate-x-1/2 rounded-full bg-primary/15 blur-3xl" }
            div { class: "relative flex flex-col items-center text-center",
                CoverCollage {
                    thumbnails,
                    class: "w-44 rounded-2xl shadow-xl sm:w-48",
                }
                div { class: "mt-5 min-w-0",
                    if let Some(eyebrow) = eyebrow {
                        div { class: "mb-1.5 flex items-center justify-center gap-1.5",
                            span { class: "size-1.5 rounded-full bg-current {accent}" }
                            p { class: "text-xs font-bold uppercase tracking-wider {accent}",
                                {eyebrow}
                            }
                        }
                    }
                    h1 {
                        class: "truncate text-2xl font-extrabold tracking-tight sm:text-3xl",
                        title: name.clone(),
                        {name.clone()}
                    }
                    p { class: "mt-1 flex flex-wrap items-center justify-center gap-1.5 text-xs text-base-content/60 sm:text-sm",
                        span {
                            "{count} "
                            if count == 1 {
                                "song"
                            } else {
                                "songs"
                            }
                        }
                        if let Some(subtitle) = subtitle {
                            span { class: "text-base-content/30", "•" }
                            span { {subtitle} }
                        }
                    }
                }
            }
            div { class: "relative mt-6 flex w-full max-w-xs flex-wrap items-center justify-center gap-3 mx-auto",
                {children}
            }
        }
    }
}
