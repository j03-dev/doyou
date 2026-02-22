use dioxus::prelude::*;

use crate::{
    Route,
    common::components::icons::{FavoriteIcon, HomeIcon, SettingIcon},
};

#[component]
pub fn Dock() -> Element {
    let current_active = use_signal(|| 0);

    let dock_item_props = &[
        (Route::Home {}, rsx!(
            HomeIcon {}
        )),
        (Route::Favorite {}, rsx!(
            FavoriteIcon {}
        )),
        (Route::Setting {}, rsx!(
            SettingIcon {}
        )),
    ];

    rsx! {
        div { class: "dock dock-lg",
            for (i , props) in dock_item_props.into_iter().enumerate() {
                DockItem {
                    index: i as i32,
                    current_active,
                    route: props.0.clone(),
                    {props.1.clone()}
                }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn DockItem(
    route: Route,
    index: i32,
    mut current_active: Signal<i32>,
    children: Element,
) -> Element {
    rsx! {
        Link {
            to: route,
            onclick: move |_| {
                current_active.set(index);
            },
            button { class: if index == current_active() { "dock-active" } else { "" }, {children} }
        }
    }
}
