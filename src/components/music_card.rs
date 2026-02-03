use dioxus::prelude::*;
use yt::data_api::types::Item;

use crate::{
    components::icons::{DownloadIcon, FavoriteIcon},
    providers::Playback,
};

#[component]
pub fn ListRowMusicCard(item: Item, index: usize) -> Element {
    let mut favorite = use_signal(|| false);
    let mut playback = use_context::<Playback>();

    let is_loading =
        use_memo(move || *playback.current_index.read() == index && *playback.is_loading.read());

    let is_playing_now =
        use_memo(move || *playback.current_index.read() == index && *playback.is_playing.read());

    let title = item.snippet.title;

    let artist = item.snippet.channel_title;

    let thumbnail = item.snippet.thumbnails.unwrap().medium.unwrap().url;

    rsx! {
        li {
            class: format!(
                "list-row{}",
                if is_playing_now() { " bg-secondary text-base-content" } else { "" },
            ),
            div {
                class: "flex-shrink-0",
                onclick: move |_| playback.start(index),
                img { class: "md:size-20 size-10 rounded-box", src: thumbnail }
            }
            div { class: "min-w-0",
                div { class: "truncate", {title} }
                div { class: "text-xs uppercase font-semibold opacity-60", {artist} }
                if is_loading() {
                    span { class: "loading loading-dots loading-sm" }
                }
            }
            button { class: "btn btn-square btn-ghost", DownloadIcon {} }
            button {
                class: "btn btn-square btn-ghost",
                onclick: move |_| favorite.set(!favorite()),
                FavoriteIcon { class: if favorite() { "fill-red-500 stroke-current-500" } else { "fill-transparent stroke-current" } }
            }
        }
    }
}
