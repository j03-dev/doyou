use crate::context::{use_favorites, use_playback};
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdDownload, LdHeart, LdPlay},
};
use yt::data_api::types::Item;

#[component]
pub fn MusicCard(item: Item, index: usize) -> Element {
    let playback = use_playback();
    let favorites = use_favorites();

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

    let favorite_item = item.clone();
    let on_favorite = move |_| {
        favorites.toggle(&favorite_item);
    };

    rsx! {
        div { class: "card bg-base-100 w-full min-w-0 rounded-xl transition-colors hover:bg-base-200/60",
            figure { class: "relative aspect-square overflow-hidden rounded-xl bg-base-200",
                img {
                    class: "size-full object-cover transition-transform duration-300 hover:scale-105",
                    src: "{thumbnail}",
                    alt: "{title}",
                    loading: "lazy",
                }
                button {
                    class: "btn btn-circle btn-md absolute bottom-2 right-2 border-none bg-primary text-primary-content shadow-md hover:scale-105 hover:bg-primary/90",
                    "aria-label": if is_playing_now() { "Playing" } else { "Play track" },
                    disabled: is_loading(),
                    onclick: move |_| playback.start(index),

                    if is_loading() {
                        span { class: "loading loading-spinner loading-sm" }
                    } else if is_playing_now() {
                        span { class: "text-sm font-bold", "Ⅱ" }
                    } else {
                        Icon { icon: LdPlay }
                    }
                }
            }
            div { class: "flex min-w-0 items-center gap-2 px-1 pt-2 pb-1",
                div { class: "flex min-w-0 flex-1 flex-col gap-1",
                    h2 {
                        class: "truncate text-sm font-semibold leading-tight",
                        title: "{title}",
                        dangerous_inner_html: title,
                    }
                    p {
                        class: "truncate text-xs text-base-content/60",
                        title: "{artist}",
                        dangerous_inner_html: artist,
                    }
                }
                div { class: "flex shrink-0 items-center gap-1",
                    button {
                        class: "btn btn-ghost btn-sm btn-square",
                        "aria-label": if is_favorite() { "Remove from favorites" } else { "Add to favorites" },
                        onclick: on_favorite,
                        Icon {
                            icon: LdHeart,
                            fill: if is_favorite() { "red" } else { "currentColor" },
                        }
                    }
                    button {
                        class: "btn btn-ghost btn-sm btn-square",
                        "aria-label": "Download track",
                        onclick: move |_| {},
                        Icon { icon: LdDownload, class: "text-base-content/70" }
                    }
                }
            }
        }
    }
}
