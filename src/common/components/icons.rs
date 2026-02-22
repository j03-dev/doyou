#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct IconProps {
    #[props(default)]
    pub class: String,
}

pub fn DoYouIcon(props: IconProps) -> Element {
    rsx!(
        p { class: "btn btn-ghost text-xl {props.class}", "DoYou" }
    )
}

pub fn PrevIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "h-6 w-6 {props.class}",
            fill: "currentColor",
            view_box: "0 0 24 24",
            path { d: "M18 18V6l-8 6 8 6zM6 6h2v12H6V6z" }
        }
    }
}

pub fn NextIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "h-6 w-6 {props.class}",
            fill: "currentColor",
            view_box: "0 0 24 24",
            path { d: "M6 6v12l8-6-8-6zM18 6h-2v12h2V6z" }
        }
    }
}

pub fn PlayIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "h-6 w-6 {props.class}",
            fill: "currentColor",
            view_box: "0 0 24 24",
            path { d: "M6 4h4v16H6zM14 4h4v16h-4z" }
        }
    }
}

pub fn PauseIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "h-6 w-6 {props.class}",
            fill: "currentColor",
            view_box: "0 0 24 24",
            path { d: "M5 3l14 9-14 9V3z" }
        }
    }
}

pub fn FavoriteIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            class: "size-[1.2em] {props.class}",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            g {
                stroke_linejoin: "round",
                stroke_linecap: "round",
                stroke_width: "2",
                fill: "none",
                stroke: "currentColor",
                path { d: "M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" }
            }
        }
    }
}

pub fn CloseIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "h-6 w-6 {props.class}",
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

pub fn DownloadIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            class: "size-[1.2em] {props.class}",
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

pub fn SearchIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "h-5 w-5 {props.class}",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z",
            }
        }
    }
}

pub fn BurgerIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "h-5 w-5 {props.class}",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M4 6h16M4 12h16M4 18h7",
            }
        }
    }
}

pub fn HomeIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            class: "size-[1.2em] {props.class}",
            "xmlns": "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            g {
                fill: "currentColor",
                stroke_linejoin: "miter",
                stroke_linecap: "butt",
                polyline {
                    points: "1 11 12 2 23 11",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_miterlimit: "10",
                    stroke_width: "2",
                }
                path {
                    d: "m5,13v7c0,1.105.895,2,2,2h10c1.105,0,2-.895,2-2v-7",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "square",
                    stroke_miterlimit: "10",
                    stroke_width: "2",
                }
                line {
                    x1: "12",
                    y1: "22",
                    x2: "12",
                    y2: "18",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "square",
                    stroke_miterlimit: "10",
                    stroke_width: "2",
                }
            }
        }
    }
}

pub fn SettingIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            class: "size-[1.2em] {props.class}",
            "xmlns": "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            g {
                fill: "currentColor",
                stroke_linejoin: "miter",
                stroke_linecap: "butt",
                circle {
                    cx: "12",
                    cy: "12",
                    r: "3",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "square",
                    stroke_miterlimit: "10",
                    stroke_width: "2",
                }
                path {
                    d: "m22,13.25v-2.5l-2.318-.966c-.167-.581-.395-1.135-.682-1.654l.954-2.318-1.768-1.768-2.318.954c-.518-.287-1.073-.515-1.654-.682l-.966-2.318h-2.5l-.966,2.318c-.581.167-1.135.395-1.654.682l-2.318-.954-1.768,1.768.954,2.318c-.287.518-.515,1.073-.682,1.654l-2.318.966v2.5l2.318.966c.167.581.395,1.135.682,1.654l-.954,2.318,1.768,1.768,2.318-.954c.518.287,1.073.515,1.654.682l.966,2.318h2.5l.966-2.318c.581-.167,1.135-.395,1.654-.682l2.318.954,1.768-1.768-.954-2.318c.287-.518.515-1.073.682-1.654l2.318-.966Z",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "square",
                    stroke_miterlimit: "10",
                    stroke_width: "2",
                }
            }
        }
    }
}
