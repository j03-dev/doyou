#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    #[props(extends = GlobalAttributes, extends=input)]
    pub attributes: Vec<Attribute>,
    #[props(default)]
    pub onclick: EventHandler<MouseEvent>,
    #[props(default)]
    pub children: Element,
}

pub fn ButtonGhost(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: "btn btn-ghost btn-circle",
            onclick: props.onclick,
            ..props.attributes,
            {props.children}
        }
    }
}
