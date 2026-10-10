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

    let play = move |_: Event<MouseData>| match on_play {
        Some(handler) => handler.call(index),
        None => playback.start(index),
    };

    rsx! {
        li { class: if is_playing_now() { "list-row items-center gap-3 rounded-xl bg-primary/10" } else { "list-row items-center gap-3 rounded-xl transition-colors hover:bg-base-200/60" },
            div { class: "w-5 shrink-0 text-center text-xs font-medium tabular-nums text-base-content/40",
                "{rank.unwrap_or(index + 1)}"
            }
            div { class: "relative shrink-0 cursor-pointer", onclick: play,
                img {
                    class: "size-12 rounded-xl object-cover md:size-14",
                    src: thumbnail,
                    alt: "{title}",
                    loading: "lazy",
                }
            }
            div { class: "min-w-0 flex-1 cursor-pointer", onclick: play,
                div {
                    class: if is_playing_now() { "truncate font-semibold text-primary" } else { "truncate font-medium" },
                    dangerous_inner_html: title,
                }
                div {
                    class: "text-xs uppercase font-semibold opacity-60",
                    dangerous_inner_html: artist,
                }
            }
            div { class: "flex shrink-0 items-center gap-0.5",
                if is_loading() {
                    span { class: "loading loading-dots loading-sm text-primary" }
                } else if is_playing_now() {
                    span { class: "loading loading-bars loading-sm text-primary" }
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
                ButtonGhost { onclick: move |evt| handler.call(evt),
                    Icon { icon: LdTrash2, class: "text-error" }
                }
            }
        }
    }
}
