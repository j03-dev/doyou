use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdDownload, LdHeart},
};
use yt::data_api::types::Item;

use crate::components::button::ButtonGhost;
use crate::context::{use_favorites, use_playback};

#[component]
pub fn MusicRow(item: Item, index: usize) -> Element {
    let playback = use_playback();
    let favorites = use_favorites();

    let item_id = item.id.as_string().unwrap();

    let is_playing_now = use_memo({
        let item_id = item_id.clone();
        move || {
            playback
                .playing
                .read()
                .as_ref()
                .map(|i| i.id.as_string().unwrap())
                == Some(item_id.clone())
                && *playback.is_playing.read()
        }
    });

    let is_loading = use_memo({
        let item_id = item_id.clone();
        move || {
            playback
                .playing
                .read()
                .as_ref()
                .map(|i| i.id.as_string().unwrap())
                == Some(item_id.clone())
                && *playback.is_loading.read()
        }
    });

    let is_favorite = use_memo({
        let item_id = item_id.clone();
        move || {
            favorites
                .tracks
                .read()
                .iter()
                .any(|t| t.track_id == item_id)
        }
    });

    let title = item.snippet.title.clone();
    let artist = item.snippet.channel_title.clone();
    let thumbnail = item.snippet.thumbnails.high.url.clone();

    let set_favorite = move |_: Event<MouseData>| {
        favorites.toggle(&item);
    };

    rsx! {
        li {
            class: format!(
                "list-row {}",
                if is_playing_now() { "bg-secondary text-base-content" } else { "" },
            ),
            div {
                class: "flex-shrink-0",
                onclick: move |_| playback.start(index),
                img { class: "md:size-20 size-10 rounded-box", src: thumbnail }
            }
            div { class: "min-w-0",
                div { class: "truncate", dangerous_inner_html: title }
                div {
                    class: "text-xs uppercase font-semibold opacity-60",
                    dangerous_inner_html: artist,
                }
                if is_loading() {
                    span { class: "loading loading-dots loading-sm" }
                }
            }
            ButtonGhost {
                Icon { icon: LdDownload }
            }
            ButtonGhost { onclick: set_favorite,
                Icon {
                    icon: LdHeart,
                    fill: if is_favorite() { "red" } else { "currentColor" },
                }
            }
        }
    }
}
