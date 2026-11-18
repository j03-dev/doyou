use dioxus::prelude::*;

#[component]
pub fn NavBar(#[props(default)] children: Element) -> Element {
    rsx! {
        nav { class: "navbar sticky top-0 z-40 min-h-16 border-b border-base-content/10 bg-base-100 px-2 shadow-sm sm:px-4",
            {children}
        }
    }
}

#[component]
pub fn NavBarItem(position: NavBarPos, #[props(default)] children: Element) -> Element {
    rsx! {
        div { class: "{position}", {children} }
    }
}

#[component]
pub fn BrandTitle(title: &'static str) -> Element {
    rsx! {
        div { class: "flex min-w-0 items-baseline gap-2",
            span { class: "truncate text-lg font-extrabold tracking-tight text-primary sm:text-xl",
                "DoYou"
            }
            span { class: "text-base-content/30", "•" }
            h1 { class: "truncate text-sm font-semibold text-base-content sm:text-base",
                {title}
            }
        }
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
