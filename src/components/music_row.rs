use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{
        LdCircleCheck, LdDownload, LdEllipsis, LdHeart, LdListPlus, LdMusic, LdTrash2,
    },
};
use yt::data_api::types::Item;

use crate::components::button::ButtonGhost;
use crate::context::{use_downloads, use_favorites, use_playback, use_playlists};
use crate::repository::TrackSource;

#[component]
pub fn MusicRow(
    item: Item,
    index: usize,
    #[props(default)] rank: Option<usize>,
    #[props(default)] on_play: Option<EventHandler<usize>>,
    #[props(default)] on_remove: Option<EventHandler<MouseEvent>>,
    #[props(default)] meta: Option<String>,
) -> Element {
    let playback = use_playback();
    let favorites = use_favorites();
    let playlists = use_playlists();
    let downloads = use_downloads();

    let item_id = item.id.as_string().unwrap_or_default();
    let menu_id = format!("music-menu-{index}");
    let anchor = format!("--music-menu-{index}");

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

    let playlist_item = item.clone();
    let download_item = item.clone();

    let set_favorite = move |_| {
        favorites.toggle(&item);
    };

    let play = move |_| match on_play {
        Some(handler) => handler.call(index),
        None => playback.start(index),
    };

    rsx! {
        li { class: if is_playing_now() { "flex h-16 w-full min-w-0 flex-nowrap items-center gap-3 overflow-hidden rounded-xl bg-primary/10 px-2 transition-colors" } else { "flex h-16 w-full min-w-0 flex-nowrap items-center gap-3 overflow-hidden rounded-xl px-2 transition-colors hover:bg-base-200/60" },
            div {
                class: "size-12 shrink-0 cursor-pointer overflow-hidden rounded-lg shadow-sm",
                onclick: play,

                if thumbnail.is_empty() {
                    div { class: "flex size-full items-center justify-center bg-base-300 text-base-content/50",
                        Icon { icon: LdMusic, class: "size-6" }
                    }
                } else {
                    img {
                        class: "block size-full object-cover",
                        src: thumbnail,
                        alt: "{title}",
                        loading: "lazy",
                    }
                }
            }
            div {
                class: "flex min-w-0 flex-1 cursor-pointer flex-col justify-center gap-1 overflow-hidden",
                onclick: play,

                div { class: "flex min-w-0 items-center gap-2",
                    span {
                        class: if is_playing_now() { "truncate text-sm font-bold leading-5 text-primary" } else { "truncate text-sm font-bold leading-5" },
                        dangerous_inner_html: title,
                    }
                }
                div { class: "flex min-w-0 items-center gap-1.5",
                    span {
                        class: "truncate text-xs text-base-content/60",
                        dangerous_inner_html: artist,
                    }
                    if let Some(meta) = meta.as_deref() {
                        span { class: "shrink-0 rounded bg-base-200 px-1 text-[10px] text-base-content/50", "{meta}" }
                    }
                }
            }
            div { class: "flex w-5 shrink-0 items-center justify-center",
                if is_loading() {
                    span { class: "loading loading-dots loading-sm text-primary" }
                } else if is_playing_now() {
                    span { class: "loading loading-bars loading-sm text-primary" }
                }
            }
            div { class: "flex shrink-0 items-center gap-0.5",
                if is_youtube {
                    ButtonGhost {
                        "aria-label": "Download track",
                        disabled: download_pending() || already_downloaded(),
                        onclick: move |_| downloads.download(&download_item),
                        if download_pending() {
                            span { class: "loading loading-spinner loading-sm" }
                        } else if already_downloaded() {
                            Icon { icon: LdCircleCheck, class: "text-success" }
                        } else {
                            Icon { icon: LdDownload }
                        }
                    }
                }
                ButtonGhost { onclick: set_favorite,
                    Icon {
                        icon: LdHeart,
                        fill: "currentColor",
                        class: if is_favorite() { "fill-error stroke-error" } else { "" },
                    }
                }
                div { class: "relative shrink-0",
                    button {
                        class: "btn btn-ghost btn-circle",
                        style: "anchor-name:{anchor}",
                        "popovertarget": menu_id.clone(),
                        Icon { icon: LdEllipsis }
                    }
                    div {
                        class: "dropdown dropdown-end w-48 rounded-box border border-base-300 bg-base-100 p-1 shadow-lg",
                        "popover": "auto",
                        id: menu_id.clone(),
                        style: "position-anchor:{anchor}",
                        button {
                            class: "flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-sm hover:bg-base-200",
                            "popovertarget": menu_id.clone(),
                            "popovertargetaction": "hide",
                            onclick: move |_| {
                                playlists.open_add(playlist_item.clone());
                                spawn(async move {
                                    let _ = document::eval(
                                            "document.getElementById('add_to_playlist')?.showModal()",
                                        )
                                        .await;
                                });
                            },
                            Icon { icon: LdListPlus }
                            "Add to playlist"
                        }
                        if let Some(handler) = on_remove {
                            button {
                                class: "flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-sm text-error hover:bg-error/10",
                                "popovertarget": menu_id.clone(),
                                "popovertargetaction": "hide",
                                onclick: move |evt| {
                                    handler.call(evt);
                                },

                                Icon { icon: LdTrash2 }
                                "Delete"
                            }
                        }
                    }
                }
            }
        }
    }
}
