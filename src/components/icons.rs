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
pub fn FavoriteIcon(#[props(default = "".to_string())] class: String) -> Element {
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

#[component]
pub fn DownloadIcon() -> Element {
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
                path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                polyline { points: "7 10 12 15 17 10" }
                line {
                    x1: "12",
                    y1: "15",
                    x2: "12",
                    y2: "3",
                }
            }
        }
    }
}

#[component]
pub fn LoadingSpinner() -> Element {
    rsx! {
        span { class: "loading loading-spinner" }
    }
}

#[component]
pub fn SunIcon() -> Element {
    rsx! {
        svg {
            class: "swap-off h-10 w-10 fill-current",
            xmlns: "http://wwww.w3.org/2000/svg",
            view_box: "0 0 24 24",
            path { d: "M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" }
        }
    }
}

#[component]
pub fn MoonIcon() -> Element {
    rsx! {
        svg {
            class: "swap-on h-10 w-10 fill-current",
            xmlns: "http://wwww.w3.org/2000/svg",
            view_box: "0 0 24 24",
            path { d: "M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" }
        }
    }
}
