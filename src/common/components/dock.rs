use dioxus::prelude::*;

use crate::{
    Route,
    common::components::icons::{FavoriteIcon, HomeIcon, SettingIcon},
};

#[component]
pub fn Dock() -> Element {
    rsx! {
        div { class: "dock dock-lg",
            DockItem { route: Route::Home {}, HomeIcon {} }
            DockItem { route: Route::Favorite {},
                FavoriteIcon { class: "fill-transparent stroke-current" }
            }
            DockItem { route: Route::Setting {}, SettingIcon {} }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn DockItem(route: Route, children: Element) -> Element {
    rsx! {
        Link { to: route, active_class: "dock-active", {children} }
    }
}
