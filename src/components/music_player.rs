use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{
    LdEllipsis, LdHeart, LdListPlus, LdMusic, LdPause, LdPlay, LdShuffle, LdSkipBack,
    LdSkipForward, LdVolume2, LdX,
};

use crate::components::alert::Alert;
use crate::context::playback::PlaybackContext;
use crate::context::{use_favorites, use_playback, use_playlists};
use crate::repository::TrackSource;

#[component]
pub fn MusicPlayer() -> Element {
    let playback = use_playback();
    let playback_error = playback.error;
    let mut show_full_player = use_signal(|| false);

    rsx! {
        div { class: "hidden",
            audio {
                id: playback.id,
                onended: move |_| playback.playback_controller(1),
                ontimeupdate: move |_| playback.update_current_time(),
                ondurationchange: move |_| playback.update_duration(),
            }
        }
        if playback.playing.read().is_some() {
            if show_full_player() {
                FullMusicPlayer { on_close_full_player: move |_| show_full_player.set(false) }
            } else {
                div { class: "fixed bottom-16 left-0 right-0 z-40 px-3 pb-2 pointer-events-none",
                    if let Some(alert_props) = playback_error() {
                        Alert { ..alert_props }
                    }
                    div { class: "pointer-events-auto overflow-hidden rounded-2xl border border-base-content/10 bg-base-100/95 shadow-lg backdrop-blur-xl",
                        MiniMusicPlayer { on_open_full_player: move |_| show_full_player.set(true) }
                    }
                }
            }
        }
    }
}

#[component]
fn FullMusicPlayer(on_close_full_player: EventHandler<MouseEvent>) -> Element {
    let playback = use_playback();
    let playback_error = playback.error;
    let favorites = use_favorites();
    let playlists = use_playlists();

    let (thumbnail, title, artist, source_label, item) = {
        let playing = playback.playing.read();
        let item = playing.clone();
        let source_label = item
            .as_ref()
            .and_then(|current| current.id.as_string())
            .map(|id| {
                if TrackSource::from_id(&id).is_local() {
                    "Local Music".to_string()
                } else {
                    "YouTube".to_string()
                }
            })
            .unwrap_or_else(|| "your library".to_string());
        (
            item.as_ref()
                .map(|current| current.snippet.thumbnails.high.url.clone())
                .unwrap_or_default(),
            item.as_ref()
                .map(|current| current.snippet.title.clone())
                .unwrap_or_else(|| "Unknown title".to_string()),
            item.as_ref()
                .map(|current| current.snippet.channel_title.clone())
                .unwrap_or_else(|| "Unknown artist".to_string()),
            source_label,
            item,
        )
    };

    let favorite_item = item.clone();
    let is_favorite = use_memo(move || {
        favorite_item
            .as_ref()
            .is_some_and(|current| favorites.is_favorite_item(current))
    });

    let toggle_item = item.clone();
    let on_favorite = move |_| {
        if let Some(current) = toggle_item.clone() {
            favorites.toggle(&current);
        }
    };

    let playlist_item = item.clone();
    let on_add_to_playlist = move |_| {
        if let Some(current) = playlist_item.clone() {
            playlists.open_add(current);
            spawn(async move {
                let _ = document::eval("add_to_playlist.showModal()").await;
            });
        }
    };

    let shuffle_playback = playback;
    let on_shuffle = move |_| {
        let queued = shuffle_playback.queue.read().clone();
        shuffle_playback.start_shuffled(queued);
    };

    rsx! {
        div { class: "fixed inset-0 z-50 flex flex-col bg-base-100",
            if let Some(alert_props) = playback_error() {
                Alert { ..alert_props }
            }
            header { class: "flex h-16 shrink-0 items-center justify-between px-3 sm:px-4",
                button {
                    class: "btn btn-ghost btn-circle rounded-full",
                    "aria-label": "Collapse player",
                    onclick: on_close_full_player,
                    Icon { icon: LdX, class: "size-7" }
                }
                div { class: "flex min-w-0 flex-1 flex-col items-center px-2 text-center",
                    span { class: "truncate text-[10px] font-bold uppercase tracking-widest text-base-content/50",
                        "Playing from {source_label}"
                    }
                    h1 { class: "truncate text-sm font-semibold", "Now Playing" }
                }
                button {
                    class: "btn btn-ghost btn-circle rounded-full",
                    "aria-label": "Track options",
                    onclick: on_add_to_playlist.clone(),
                    Icon { icon: LdEllipsis }
                }
            }
            main { class: "flex-1 overflow-y-auto px-4 pb-8 pt-2 sm:pt-4",
                div { class: "mx-auto flex w-full max-w-sm flex-col",
                    div { class: "relative mx-auto aspect-square w-full max-w-xs overflow-hidden rounded-3xl bg-base-300 shadow-2xl",
                        div { class: "absolute -inset-3 -z-10 rounded-3xl bg-primary/25 blur-3xl" }
                        if thumbnail.is_empty() {
                            div { class: "flex size-full items-center justify-center bg-base-300 text-base-content/40",
                                Icon { icon: LdMusic, class: "size-16" }
                            }
                        } else {
                            img {
                                class: "size-full object-cover",
                                src: thumbnail,
                                alt: "Album cover",
                            }
                        }
                        div { class: "absolute left-3 top-3 flex items-center gap-1.5 rounded-full border-none bg-base-100/85 px-3 py-1 text-[10px] font-bold uppercase tracking-wider shadow-sm backdrop-blur",
                            span { class: "size-2 rounded-full bg-success animate-pulse" }
                            if is_favorite() {
                                "Favorite"
                            } else {
                                {source_label.clone()}
                            }
                        }
                        div { class: "absolute inset-x-0 bottom-0 flex items-end justify-between bg-gradient-to-t from-black/60 to-transparent px-3 pb-3 pt-10 text-white/90",
                            div { class: "flex items-center gap-1.5 text-xs font-semibold",
                                Icon { icon: LdVolume2, class: "size-4" }
                                "DoYou Audio"
                            }
                            span { class: "rounded-full bg-white/15 px-2 py-0.5 text-[10px] font-bold uppercase tracking-wider",
                                {source_label}
                            }
                        }
                    }

                    div { class: "mt-6 flex w-full items-center justify-between gap-3",
                        div { class: "min-w-0 flex-1",
                            h2 {
                                class: "truncate text-xl font-bold tracking-tight",
                                dangerous_inner_html: title,
                            }
                            p {
                                class: "mt-1 truncate text-sm text-base-content/60",
                                dangerous_inner_html: artist,
                            }
                        }
                        div { class: "flex shrink-0 items-center gap-2",
                            button {
                                class: format!(
                                    "btn btn-circle border-none shadow-sm {}",
                                    if is_favorite() { "bg-primary/10 text-primary" } else { "bg-base-200 text-base-content/70" },
                                ),
                                "aria-label": if is_favorite() { "Remove from favorites" } else { "Add to favorites" },
                                onclick: on_favorite,
                                Icon {
                                    icon: LdHeart,
                                    fill: "currentColor",
                                    class: if is_favorite() { "fill-error stroke-error" } else { "" },
                                }
                            }
                            button {
                                class: "btn btn-circle border-none bg-base-200 text-base-content/70 shadow-sm",
                                "aria-label": "Add to playlist",
                                onclick: on_add_to_playlist,
                                Icon { icon: LdListPlus }
                            }
                        }
                    }

                    div { class: "mt-7 w-full",
                        ProgressBar { playback, show_remaining: true }
                    }

                    div { class: "mt-7 flex w-full items-center justify-between gap-2 px-1",
                        button {
                            class: "btn btn-ghost btn-circle rounded-full text-base-content/60",
                            "aria-label": "Shuffle queue",
                            onclick: on_shuffle,
                            Icon { icon: LdShuffle, class: "size-6" }
                        }
                        button {
                            class: "btn btn-circle border-none bg-base-200 shadow-sm",
                            "aria-label": "Previous track",
                            onclick: move |_| playback.playback_controller(-1),
                            Icon { icon: LdSkipBack, class: "size-7" }
                        }
                        if *playback.is_loading.read() {
                            button { class: "btn btn-primary btn-circle btn-xl",
                                span { class: "loading loading-spinner" }
                            }
                        } else {
                            button {
                                class: "btn btn-primary btn-circle btn-xl shadow-lg",
                                "aria-label": if *playback.is_playing.read() { "Pause" } else { "Play" },
                                onclick: move |_| playback.toggle_play(),
                                if *playback.is_playing.read() {
                                    Icon { icon: LdPause, class: "size-8" }
                                } else {
                                    Icon { icon: LdPlay, class: "size-8" }
                                }
                            }
                        }
                        button {
                            class: "btn btn-circle border-none bg-base-200 shadow-sm",
                            "aria-label": "Next track",
                            onclick: move |_| playback.playback_controller(1),
                            Icon { icon: LdSkipForward, class: "size-7" }
                        }
                        div { class: "size-12 shrink-0" }
                    }
                }
            }
        }
    }
}

#[component]
fn MiniMusicPlayer(on_open_full_player: EventHandler<()>) -> Element {
    let playback = use_playback();

    let playing = playback.playing.read();
    let thumbnail = playing
        .as_ref()
        .map(|item| item.snippet.thumbnails.high.url.clone())
        .unwrap_or_default();
    let title = playing
        .as_ref()
        .map(|item| item.snippet.title.clone())
        .unwrap_or_else(|| "Unknown title".to_string());
    let artist = playing
        .as_ref()
        .map(|item| item.snippet.channel_title.clone())
        .unwrap_or_else(|| "Unknown artist".to_string());

    let current_time = *playback.current_time.read();
    let duration = *playback.duration.read();
    let progress = if duration > 0.0 {
        (current_time / duration * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };

    rsx! {
        div { class: "relative flex items-center gap-3 p-2.5",
            div { class: "absolute inset-x-0 top-0 h-1 overflow-hidden rounded-full bg-base-300",
                div { class: "h-full rounded-full bg-primary", style: "width: {progress}%" }
            }
            div {
                class: "flex min-w-0 flex-1 cursor-pointer items-center gap-3",
                onclick: move |_| on_open_full_player.call(()),
                if thumbnail.is_empty() {
                    div { class: "flex size-10 shrink-0 items-center justify-center rounded-xl bg-base-300 text-base-content/50",
                        Icon { icon: LdMusic, class: "size-5" }
                    }
                } else {
                    img {
                        class: "size-10 shrink-0 rounded-xl object-cover",
                        src: thumbnail,
                        alt: "Album cover",
                    }
                }
                div { class: "min-w-0",
                    h3 {
                        class: "truncate text-sm font-bold",
                        dangerous_inner_html: title,
                    }
                    p {
                        class: "truncate text-xs text-base-content/60",
                        dangerous_inner_html: artist,
                    }
                }
            }
            div { class: "flex shrink-0 items-center gap-1",
                button {
                    class: "btn btn-ghost btn-circle btn-sm",
                    "aria-label": "Previous track",
                    onclick: move |_| playback.playback_controller(-1),
                    Icon { icon: LdSkipBack }
                }
                if *playback.is_loading.read() {
                    button { class: "btn btn-primary btn-circle btn-sm",
                        span { class: "loading loading-spinner loading-xs" }
                    }
                } else {
                    button {
                        class: "btn btn-primary btn-circle btn-sm shadow-md",
                        "aria-label": if *playback.is_playing.read() { "Pause" } else { "Play" },
                        onclick: move |_| playback.toggle_play(),
                        if *playback.is_playing.read() {
                            Icon { icon: LdPause }
                        } else {
                            Icon { icon: LdPlay }
                        }
                    }
                }
                button {
                    class: "btn btn-ghost btn-circle btn-sm",
                    "aria-label": "Next track",
                    onclick: move |_| playback.playback_controller(1),
                    Icon { icon: LdSkipForward }
                }
            }
        }
    }
}

#[component]
fn ProgressBar(playback: PlaybackContext, #[props(default)] show_remaining: bool) -> Element {
    let current_time = playback.current_time.read();
    let duration = playback.duration.read();

    let format_time = |seconds: f64| {
        let total = seconds.floor().max(0.0) as u64;
        format!("{:02}:{:02}", total / 60, total % 60)
    };
    let remaining = (*duration - *current_time).max(0.0);

    rsx! {
        div { class: "w-full",
            div { class: "py-2",
                progress {
                    class: "progress progress-primary h-2 w-full rounded-full",
                    value: current_time.to_string(),
                    max: duration.to_string(),
                }
            }
            div { class: "flex items-center justify-between text-[11px] font-mono text-base-content/60",
                span { class: "font-semibold", {format_time(*current_time)} }
                if show_remaining {
                    span { "-{format_time(remaining)}" }
                } else {
                    span { {format_time(*duration)} }
                }
            }
        }
    }
}
