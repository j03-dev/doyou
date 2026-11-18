use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdCompass, LdDownload, LdHeart, LdSettings},
};

use crate::route::Route;

#[component]
pub fn Dock() -> Element {
    rsx! {
        Outlet::<Route> {}
        div { class: "dock dock-lg border-t border-base-content/10 bg-base-100/90 backdrop-blur-xl",
            DockItem { route: Route::Home {}, label: "Explore",
                Icon { icon: LdCompass }
            }
            DockItem { route: Route::Favorite {}, label: "Favorites",
                Icon { icon: LdHeart }
            }
            DockItem { route: Route::Downloads {}, label: "Downloads",
                Icon { icon: LdDownload }
            }
            DockItem { route: Route::Setting {}, label: "Settings",
                Icon { icon: LdSettings }
            }
        }
    }
}

#[component]
fn DockItem(route: Route, label: &'static str, children: Element) -> Element {
    rsx! {
        Link {
            to: route,
            class: "gap-0.5",
            active_class: "dock-active text-primary",
            {children}
            span { class: "dock-label text-[10px] font-semibold", {label} }
        }
    }
}
