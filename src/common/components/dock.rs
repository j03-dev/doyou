use dioxus::prelude::*;

use crate::Route;
use crate::common::components::icons::{FavoriteIcon, HomeIcon, SettingIcon};

#[component]
pub fn Dock() -> Element {
    rsx! {
        Outlet::<Route> {}
        div { class: "dock dock-lg",
            DockItem { route: Route::Home {}, HomeIcon {} }
            DockItem { route: Route::Favorite {},
                FavoriteIcon { class: "fill-transparent stroke-current" }
            }
            DockItem { route: Route::Setting {}, SettingIcon {} }
        }
    }
}

#[component]
fn DockItem(route: Route, children: Element) -> Element {
    rsx! {
        Link { to: route, active_class: "dock-active", {children} }
    }
}
