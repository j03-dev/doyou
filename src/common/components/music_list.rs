use dioxus::prelude::*;
use yt::data_api::types::Item;

use crate::common::components::button::ButtonGhost;
use crate::common::components::icons::{DownloadIcon, FavoriteIcon};
use crate::common::context::{use_favorites, use_playback};
use crate::core::db::models::YoutubeTrack;

#[component]
pub fn MusicList(items: Vec<Item>) -> Element {
    let mut playback = use_playback();
    playback.queue.set(items.clone());

    rsx! {
        ul { class: "list bg-base-100 rounded-box shadow-md",
            for (index , item) in items.iter().enumerate() {
                MusicCard { item: item.clone(), index }
            }
        }
    }
}

#[component]
fn MusicCard(item: Item, index: usize) -> Element {
    let playback = use_playback();
    let mut favorites = use_favorites();

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
        move || favorites.tracks.read().iter().any(|t| t.id == item_id)
    });

    let item_id = item_id.clone();
    let title = item.snippet.title.clone();
    let artist = item.snippet.channel_title.clone();
    let thumbnail = item.snippet.thumbnails.high.url.clone();

    let set_favorite = move |_: Event<MouseData>| {
        let item_id = item_id.clone();
        let title = item.snippet.title.clone();
        let artist = item.snippet.channel_title.clone();
        let thumbnail = item.snippet.thumbnails.high.url.clone();
        spawn(async move {
            let is_fav = favorites.tracks.read().iter().any(|t| t.id.eq(&item_id));
            if !is_fav {
                let track = YoutubeTrack {
                    id: item_id.clone(),
                    title: title.to_string(),
                    channel_name: artist.to_string(),
                    thumbnail_url: thumbnail.to_string(),
                };
                favorites.add(track).await.ok();
            } else {
                favorites.remove(&item_id).await.ok();
            }
        });
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
            ButtonGhost { DownloadIcon {} }
            ButtonGhost { onclick: set_favorite,
                FavoriteIcon { class: if is_favorite() { "fill-error stroke-error" } else { "fill-transparent stroke-current" } }
            }
        }
    }
}
