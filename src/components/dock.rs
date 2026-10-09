use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdCompass, LdHeart, LdSettings},
};

use crate::route::Route;

#[component]
pub fn Dock() -> Element {
    rsx! {
        Outlet::<Route> {}
        div { class: "dock dock-lg",
            DockItem { route: Route::Home {},
                Icon { icon: LdCompass }
            }
            DockItem { route: Route::Favorite {},
                Icon { icon: LdHeart }
            }
            DockItem { route: Route::Setting {},
                Icon { icon: LdSettings }
            }
        }
    }
}

#[component]
fn DockItem(route: Route, children: Element) -> Element {
    rsx! {
        Link { to: route, active_class: "dock-active", {children} }
    }
}
