use dioxus::prelude::*;
use yt::data_api::types::Item;

use crate::common::components::button::IconButton;
use crate::common::components::icons::{DownloadIcon, FavoriteIcon};
use crate::common::context::{use_alert, use_favorites};
use crate::core::db;
use crate::core::db::models::YoutubeTrack;
use crate::core::playback::Playback;

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
    let mut playback = use_context::<Playback>();
    let alert = use_alert();
    let favorites = use_favorites();

    let is_loading =
        use_memo(move || *playback.current_index.read() == index && *playback.is_loading.read());

    let is_playing_now =
        use_memo(move || *playback.current_index.read() == index && *playback.is_playing.read());

    let fav_tracks = favorites.tracks;
    let is_favorite = use_memo({
        let tracks = fav_tracks;
        let item_id = item.id.as_string().unwrap();
        move || tracks.read().iter().any(|t| t.id == item_id)
    });

    let title = Signal::new(item.snippet.title);
    let artist = Signal::new(item.snippet.channel_title);
    let thumbnail = Signal::new(item.snippet.thumbnails.default.url);
    let youtube_video_id = Signal::new(item.id.as_string().unwrap());

    let fav_tracks_handler = favorites.tracks;
    let alert_handler = alert.message;
    let set_favorite = move |_: Event<MouseData>| {
        let title = title();
        let channel_name = artist();
        let thumbnail_url = thumbnail();
        let video_id = youtube_video_id();
        let mut tracks = fav_tracks_handler;
        let mut msg = alert_handler;

        spawn(async move {
            let is_fav = tracks.read().iter().any(|t| t.id == video_id);
            if !is_fav {
                let track = YoutubeTrack {
                    id: video_id,
                    title,
                    channel_name,
                    thumbnail_url,
                };
                if let Err(err) = db::add_to_favorite(track.clone()).await {
                    msg.set(Some(err.to_string()));
                } else {
                    tracks.write().push(track);
                }
            } else if let Err(err) = db::remove_from_favorite(&video_id).await {
                msg.set(Some(err.to_string()));
            } else {
                tracks.write().retain(|t| t.id != video_id);
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
            IconButton { DownloadIcon {} }
            IconButton { on_click: set_favorite,
                FavoriteIcon { class: if is_favorite() { "fill-red-500 stroke-current-500" } else { "fill-transparent stroke-current" } }
            }
        }
    }
}
