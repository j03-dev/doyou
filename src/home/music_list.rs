use dioxus::prelude::*;
use yt::data_api::types::Item;

use crate::common::components::icons::{DownloadIcon, FavoriteIcon};
use crate::common::components::button::Button;
use crate::common::providers::Playback;

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

    let is_loading =
        use_memo(move || *playback.current_index.read() == index && *playback.is_loading.read());

    let is_playing_now =
        use_memo(move || *playback.current_index.read() == index && *playback.is_playing.read());

    let title = item.snippet.title;
    let artist = item.snippet.channel_title;
    let thumbnail = item.snippet.thumbnails.default.url;

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
            Button { DownloadIcon {} }
            Button { on_click: move |_| favorite.set(!favorite()),
                FavoriteIcon { class: if favorite() { "fill-red-500 stroke-current-500" } else { "fill-transparent stroke-current" } }
            }
        }
    }
}
