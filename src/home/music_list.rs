use dioxus::prelude::*;
use yt::data_api::types::Item;

use crate::common::components::button::IconButton;
use crate::common::components::icons::{DownloadIcon, FavoriteIcon};
use crate::common::context::{use_alert, use_favorites, use_playback};
use crate::core::db::models::YoutubeTrack;

#[component]
pub fn MusicList(items: Signal<Vec<Item>>) -> Element {
    rsx! {
        ul { class: "list bg-base-100 rounded-box shadow-md",
            for (index , item) in items.read().iter().enumerate() {
                MusicCard { item: item.clone(), index }
            }
        }
    }
}

#[component]
fn MusicCard(item: Item, index: usize) -> Element {
    let playback = use_playback();
    let mut alert = use_alert();
    let favorites = use_favorites();

    use_effect(move || {
        if let Some(err_msg) = favorites.error.read().as_ref() {
            alert.message.set(Some(err_msg.clone()));
        }
    });

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
        let item_id = item.id.as_string().unwrap();
        move || favorites.tracks.read().iter().any(|t| t.id == item_id)
    });

    let title = item.snippet.title.clone();
    let artist = item.snippet.channel_title.clone();
    let thumbnail = item.snippet.thumbnails.high.url.clone();
    let video_id = item.id.as_string().unwrap();

    let set_favorite_title = title.clone();
    let set_favorite_artist = artist.clone();
    let set_favorite_thumbnail = thumbnail.clone();
    let set_favorite_video_id = video_id.clone();

    let set_favorite = move |_: Event<MouseData>| {
        let favorites = use_favorites();
        let is_fav = favorites
            .tracks
            .read()
            .iter()
            .any(|t| t.id == set_favorite_video_id);
        if !is_fav {
            let track = YoutubeTrack {
                id: set_favorite_video_id.clone(),
                title: set_favorite_title.clone(),
                channel_name: set_favorite_artist.clone(),
                thumbnail_url: set_favorite_thumbnail.clone(),
            };
            favorites.add(track);
        } else {
            favorites.remove(&set_favorite_video_id);
        }
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
            IconButton { DownloadIcon {} }
            IconButton { on_click: set_favorite,
                FavoriteIcon { class: if is_favorite() { "fill-error stroke-error" } else { "fill-transparent stroke-current" } }
            }
        }
    }
}
