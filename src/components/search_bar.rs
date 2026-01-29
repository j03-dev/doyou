use dioxus::prelude::*;

#[component]
pub fn SearchBar(mut on_search: EventHandler<Event<FormData>>) -> Element {
    rsx! {
        form { class: "flex flex-row justify-center gap-2", onsubmit: on_search,
            label { class: "input input-primary",
                svg {
                    class: "h-[1em] opacity-50",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    g {
                        stroke_linejoin: "round",
                        stroke_linecap: "round",
                        fill: "none",
                        stroke: "currentColor",
                        circle { cx: "11", cy: "11", r: "8" }
                        path { d: "m21 21-4.3-4.3" }
                    }
                }
                input {
                    name: "search",
                    r#type: "search",
                    class: "grow",
                    placeholder: "Search",
                }
            }
        }
    }
}
