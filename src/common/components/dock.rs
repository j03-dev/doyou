use dioxus::prelude::*;

use crate::{
    Route,
    common::components::{
        icons::{FavoriteIcon, HomeIcon, SettingIcon},
        music_player::MusicPlayer,
    },
    core::playback::Playback,
};

#[component]
pub fn Dock() -> Element {
    let mut playback = use_context::<Playback>();

    rsx! {
        Outlet::<Route> {}
        div { class: "hidden",
            audio {
                id: playback.id,
                onended: move |_| playback.playback_controller(1),
                ontimeupdate: move |_| playback.update_current_time(),
                ondurationchange: move |_| playback.update_duration(),
            }
        }
        if playback.playing.read().is_some() {
            MusicPlayer {}
        }
        div { class: "dock dock-lg",
            DockItem { route: Route::Home {}, HomeIcon {} }
            DockItem { route: Route::FavoriteList {},
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
