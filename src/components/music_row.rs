use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdDownload, LdEllipsis, LdHeart, LdTrash2},
};
use yt::data_api::types::Item;

use crate::components::button::ButtonGhost;
use crate::context::{use_favorites, use_playback, use_playlists};

#[component]
pub fn MusicRow(
    item: Item,
    index: usize,
    #[props(default)] rank: Option<usize>,
    #[props(default)] on_play: Option<EventHandler<usize>>,
    #[props(default)] on_remove: Option<EventHandler<MouseEvent>>,
) -> Element {
    let playback = use_playback();
    let favorites = use_favorites();
    let playlists = use_playlists();

    let item_id = item.id.as_string().unwrap_or_default();

    let is_playing_now = use_memo({
        let item_id = item_id.clone();
        move || {
            playback
                .playing
                .read()
                .as_ref()
                .and_then(|i| i.id.as_string())
                .as_deref()
                == Some(item_id.as_str())
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
                .and_then(|i| i.id.as_string())
                .as_deref()
                == Some(item_id.as_str())
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

    let more_item = item.clone();
    let set_favorite = move |_: Event<MouseData>| {
        favorites.toggle(&item);
    };

    let set_playlist = move |_: Event<MouseData>| {
        playlists.open_add(more_item.clone());
        spawn(async move {
            let _ = document::eval("add_to_playlist.showModal()").await;
        });
    };

    rsx! {
        li {
            class: format!(
                "list-row {}",
                if is_playing_now() { "bg-secondary text-base-content" } else { "" },
            ),
            if let Some(rank) = rank {
                div { class: "w-6 flex-shrink-0 text-center text-sm font-bold opacity-60",
                    "{rank}"
                }
            }
            div {
                class: "flex-shrink-0 cursor-pointer",
                onclick: move |_| match on_play {
                    Some(handler) => handler.call(index),
                    None => playback.start(index),
                },
                img { class: "md:size-20 size-10 rounded-box", src: thumbnail }
            }
            div {
                class: "min-w-0 flex-1 cursor-pointer",
                onclick: move |_| match on_play {
                    Some(handler) => handler.call(index),
                    None => playback.start(index),
                },
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
                    fill: "currentColor",
                    class: if is_favorite() { "fill-error stroke-error" } else { "" },
                }
            }
            ButtonGhost { onclick: set_playlist,
                Icon { icon: LdEllipsis }
            }
            if let Some(handler) = on_remove {
                ButtonGhost {
                    onclick: move |evt| handler.call(evt),
                    Icon {
                        icon: LdTrash2,
                        class: "text-error",
                    }
                }
            }
        }
    }
}
