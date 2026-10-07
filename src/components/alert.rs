use std::time::Duration;

use dioxus::prelude::*;

use crate::context::AlertProps;

#[component]
pub fn Alert(props: AlertProps) -> Element {
    let mut visible = use_signal(|| true);

    use_effect(move || {
        spawn(async move {
            tokio::time::sleep(Duration::from_millis(props.duration)).await;
            visible.set(false);
        });
    });

    if !visible() {
        return rsx! {};
    }

    rsx! {
        div { role: "alert", class: "alert {props.level}",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "h-6 w-6 shrink-0 stroke-current",
                fill: "none",
                view_box: "0 0 24 24",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    d: "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
                }
            }
            span { {props.message} }
        }
    }
}
