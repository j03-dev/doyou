use crate::components::artwork::Artwork;
use crate::context::{use_downloads, use_favorites, use_playback, use_playlists};
use crate::repository::TrackSource;
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdCircleCheck, LdDownload, LdEllipsis, LdHeart, LdPlay},
};
use yt::data_api::types::Item;

#[component]
pub fn MusicCard(
    item: Item,
    index: usize,
    #[props(default = true)] show_extra_actions: bool,
    #[props(default)] on_play: Option<EventHandler<usize>>,
) -> Element {
    let playback = use_playback();
    let favorites = use_favorites();
    let playlists = use_playlists();
    let downloads = use_downloads();

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

    let favorite_item = item.clone();
    let is_favorite = use_memo(move || favorites.is_favorite_item(&favorite_item));

    let is_youtube = TrackSource::from_id(&item_id) == TrackSource::YouTube;

    let download_pending = use_memo({
        let video_id = item_id.clone();
        move || downloads.is_pending(&video_id)
    });

    let badge_item = item.clone();
    let already_downloaded = use_memo(move || downloads.is_downloaded_item(&badge_item));

    let title = item.snippet.title.clone();
    let artist = item.snippet.channel_title.clone();
    let thumbnail = item.snippet.thumbnails.high.url.clone();

    let toggle_item = item.clone();
    let on_favorite = move |_| {
        favorites.toggle(&toggle_item);
    };

    let more_item = item.clone();
    let on_more = move |_| {
        playlists.open_add(more_item.clone());
        spawn(async move {
            let _ = document::eval("add_to_playlist.showModal()").await;
        });
    };

    rsx! {
        div {
            class: format!(
                "card w-full min-w-0 rounded-2xl p-3 transition-all {}",
                if is_playing_now() {
                    "bg-primary/10 ring-2 ring-primary shadow-lg"
                } else {
                    "bg-base-100 shadow-sm hover:bg-base-200/60"
                },
            ),
            figure { class: "relative aspect-video overflow-hidden rounded-xl bg-base-300 shadow-inner",
                Artwork {
                    src: thumbnail,
                    alt: title.clone(),
                    class: "size-full object-cover transition-transform duration-300 hover:scale-105",
                    icon_class: "size-10",
                }
                div { class: "pointer-events-none absolute inset-0 bg-gradient-to-t from-black/50 via-transparent to-transparent" }
                if show_extra_actions {
                    button {
                        class: "btn btn-ghost btn-circle btn-sm absolute right-3 top-3 border-none bg-base-100/80 shadow-md backdrop-blur",
                        "aria-label": "Add to playlist",
                        onclick: on_more,
                        Icon { icon: LdEllipsis, class: "text-base-content/70" }
                    }
                }
                button {
                    class: "btn btn-primary btn-circle btn-md absolute bottom-3 right-3 border-none shadow-lg",
                    "aria-label": if is_playing_now() { "Playing" } else { "Play track" },
                    disabled: is_loading(),
                    onclick: move |_| match on_play {
                        Some(handler) => handler.call(index),
                        None => playback.start(index),
                    },

                    if is_loading() {
                        span { class: "loading loading-spinner loading-sm" }
                    } else if is_playing_now() {
                        span { class: "text-sm font-bold", "Ⅱ" }
                    } else {
                        Icon { icon: LdPlay }
                    }
                }
            }
            div { class: "flex min-w-0 items-center justify-between gap-3 pt-3",
                div { class: "flex min-w-0 flex-1 flex-col gap-1",
                    h2 {
                        class: "truncate text-sm font-bold leading-tight",
                        title: "{title}",
                        dangerous_inner_html: title,
                    }
                    p {
                        class: "truncate text-xs text-base-content/60",
                        title: "{artist}",
                        dangerous_inner_html: artist,
                    }
                }
                div { class: "flex shrink-0 items-center gap-0.5",
                    button {
                        class: "btn btn-ghost btn-sm btn-circle",
                        "aria-label": if is_favorite() { "Remove from favorites" } else { "Add to favorites" },
                        onclick: on_favorite,
                        Icon {
                            icon: LdHeart,
                            fill: "currentColor",
                            class: if is_favorite() { "fill-error stroke-error" } else { "" },
                        }
                    }
                    if show_extra_actions && is_youtube {
                        button {
                            class: "btn btn-ghost btn-sm btn-circle",
                            "aria-label": "Download track",
                            disabled: download_pending() || already_downloaded(),
                            onclick: move |_| downloads.download(&item),
                            if download_pending() {
                                span { class: "loading loading-spinner loading-sm" }
                            } else if already_downloaded() {
                                Icon {
                                    icon: LdCircleCheck,
                                    class: "text-success",
                                }
                            } else {
                                Icon {
                                    icon: LdDownload,
                                    class: "text-base-content/70",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
