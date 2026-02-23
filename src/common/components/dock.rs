use dioxus::prelude::*;

use crate::{
    Route,
    common::components::{
        icons::{FavoriteIcon, HomeIcon, SettingIcon},
        music_player::MusicPlayer,
    },
    common::context::use_playback,
};

#[component]
pub fn Dock() -> Element {
    let playback = use_playback();

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
