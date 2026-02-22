use dioxus::prelude::*;
use yt::data_api::types::Item;

use crate::common::components::button::IconButton;
use crate::common::components::icons::{DownloadIcon, FavoriteIcon};
use crate::core::db::{self, models::YoutubeTrack};
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
    let mut favorite = use_signal(|| false);
    let mut playback = use_context::<Playback>();
    let mut status_msg = use_context::<Signal<Option<String>>>();

    let is_loading =
        use_memo(move || *playback.current_index.read() == index && *playback.is_loading.read());

    let is_playing_now =
        use_memo(move || *playback.current_index.read() == index && *playback.is_playing.read());

    let title = Signal::new(item.snippet.title);
    let artist = Signal::new(item.snippet.channel_title);
    let thumbnail = Signal::new(item.snippet.thumbnails.default.url);
    let youtube_video_id = Signal::new(item.id.as_string().unwrap());

    use_effect(move || {
        spawn(async move {
            match db::is_favorite(&youtube_video_id()).await {
                Ok(b) => favorite.set(b),
                Err(err) => status_msg.set(Some(err.to_string())),
            }
        });
    });

    let set_favorite = move |_: Event<MouseData>| {
        let title = title();
        let channel_name = artist();
        let thumbnail_url = thumbnail();
        async move {
            if !favorite() {
                let track = YoutubeTrack {
                    id: youtube_video_id(),
                    title,
                    channel_name,
                    thumbnail_url,
                };
                if let Err(err) = db::add_to_favorite(track).await {
                    status_msg.set(Some(err.to_string()));
                    favorite.set(true);
                }
            } else {
                if let Err(err) = db::remove_from_favorite(&youtube_video_id()).await {
                    status_msg.set(Some(err.to_string()));
                    favorite.set(true);
                }
            }
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
                FavoriteIcon { class: if favorite() { "fill-red-500 stroke-current-500" } else { "fill-transparent stroke-current" } }
            }
        }
    }
}
