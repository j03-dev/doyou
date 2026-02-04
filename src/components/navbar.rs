use dioxus::prelude::*;

#[component]
pub fn NavBar(#[props(default)] children: Element) -> Element {
    rsx! {
        nav { class: "navbar", {children} }
    }
}

#[component]
pub fn NavBarElement(position: NavBarPosition, #[props(default)] children: Element) -> Element {
    let class = match position {
        NavBarPosition::Start => "navbar-start",
        NavBarPosition::Center => "navbar-center",
        NavBarPosition::End => "navbar-end",
    };

    rsx! {
        div { class, {children} }
    }
}

#[derive(PartialEq, Clone)]
pub enum NavBarPosition {
    Start,
    Center,
    End,
}
