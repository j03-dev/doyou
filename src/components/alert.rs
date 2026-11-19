use std::time::Duration;

use dioxus::prelude::*;

use crate::context::{AlertProps, use_alerts};

#[component]
pub fn Alert(props: AlertProps) -> Element {
    let seen = use_alerts();
    let id = props.id;
    let duration = props.duration;
    let mut visible = use_signal(|| true);

    use_effect(move || {
        spawn(async move {
            tokio::time::sleep(Duration::from_millis(duration)).await;
            seen.mark_seen(id);
            visible.set(false);
        });
    });

    // Navigating away before the timer ends must not bring the alert back on
    // the next visit either.
    use_drop(move || seen.mark_seen(id));

    if seen.is_seen(id) || !visible() {
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
