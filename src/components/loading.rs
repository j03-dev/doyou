#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct LoadingProps {
    #[props(default)]
    pub class: &'static str,
    #[props(default = 11)]
    pub size: i32,
}

pub fn LoadingSpinner(pros: LoadingProps) -> Element {
    rsx! {
        span { class: "loading loading-spinner {pros.class} size-{pros.size}" }
    }
}
