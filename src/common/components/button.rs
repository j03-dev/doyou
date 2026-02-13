#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    #[props(default)]
    r#type: &'static str,
    #[props(default)]
    pub class: &'static str,
    #[props(default)]
    pub on_click: EventHandler<MouseEvent>,
    pub children: Element,
}

pub fn IconButton(pros: ButtonProps) -> Element {
    rsx! {
        button {
            r#type: pros.r#type,
            class: "btn btn-ghost btn-circle {pros.class}",
            onclick: pros.on_click,
            {pros.children}
        }
    }
}

pub fn Button(pros: ButtonProps) -> Element {
    rsx! {
        button {
            r#type: pros.r#type,
            class: "btn {pros.class}",
            onclick: pros.on_click,
            {pros.children}
        }
    }
}
