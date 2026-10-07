#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TextInputPropos {
    #[props(extends = input)]
    pub attributes: Vec<Attribute>,
    #[props(default)]
    pub children: Element,
}

pub fn TextInput(props: TextInputPropos) -> Element {
    rsx! {
        div { class: "flex flex-row jusify-center w-full gap-2",
            label { class: "input input-md input-primary w-full",
                {props.children}
                input { class: "grow", ..props.attributes }
            }
        }
    }
}
