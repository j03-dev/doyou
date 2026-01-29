use dioxus::prelude::*;

#[component]
pub fn PrevIcon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "h-6 w-6",
            fill: "currentColor",
            view_box: "0 0 24 24",
            path { d: "M18 18V6l-8 6 8 6zM6 6h2v12H6V6z" }
        }
    }
}

#[component]
pub fn NextIcon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "h-6 w-6",
            fill: "currentColor",
            view_box: "0 0 24 24",
            path { d: "M6 6v12l8-6-8-6zM18 6h-2v12h2V6z" }
        }
    }
}

#[component]
pub fn PlayIcon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "h-6 w-6",
            fill: "currentColor",
            view_box: "0 0 24 24",
            path { d: "M6 4h4v16H6zM14 4h4v16h-4z" }
        }
    }
}

#[component]
pub fn PauseIcon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "h-6 w-6",
            fill: "currentColor",
            view_box: "0 0 24 24",
            path { d: "M5 3l14 9-14 9V3z" }
        }
    }
}

#[component]
pub fn FavoriteIcon(class: String) -> Element {
    rsx! {
        svg {
            class: "size-[1.2em]",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            g {
                stroke_linejoin: "round",
                stroke_linecap: "round",
                stroke_width: "2",
                fill: "none",
                stroke: "currentColor",
                class,
                path { d: "M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" }
            }
        }
    }
}

#[component]
pub fn CloseIcon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "h-6 w-6",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "2",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M6 18L18 6M6 6l12 12",
            }
        }
    }
}
