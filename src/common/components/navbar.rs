use dioxus::prelude::*;

#[component]
pub fn NavBar(#[props(default)] children: Element) -> Element {
    rsx! {
        nav { class: "navbar", {children} }
    }
}

#[component]
pub fn NavBarItem(position: NavBarPos, #[props(default)] children: Element) -> Element {
    rsx! {
        div { class: "{position}", {children} }
    }
}

#[derive(PartialEq, Clone)]
pub enum NavBarPos {
    Start,
    Center,
    End,
}

impl std::fmt::Display for NavBarPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pos = match self {
            NavBarPos::Start => "navbar-start",
            NavBarPos::Center => "navbar-center",
            NavBarPos::End => "navbar-end",
        };
        write!(f, "{}", pos)
    }
}
