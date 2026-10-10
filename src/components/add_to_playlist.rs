use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdPlus;

use crate::components::alert::Alert;
use crate::components::cover::CoverCollage;
use crate::components::form::get_value_from;
use crate::components::text_input::TextInput;
use crate::context::use_playlists;

#[component]
pub fn AddToPlaylistDialog() -> Element {
    let playlists = use_playlists();

    let pending = playlists.pending_track;
    let error = playlists.error;
    let playlist_rows = playlists.items;

    let create_playlist = move |evt: Event<FormData>| {
        evt.prevent_default();
        let name = get_value_from(evt, "playlist_name").unwrap_or_default();
        playlists.create(name);
    };

    rsx! {
        dialog { id: "add_to_playlist", class: "modal",
            div { class: "modal-box w-96 max-w-[calc(100vw-2rem)]",
                h3 { class: "font-bold text-lg", "Add to playlist" }

                if let Some(item) = pending() {
                    div { class: "flex items-center gap-3 py-2",
                        img {
                            class: "size-10 rounded-box object-cover",
                            src: item.snippet.thumbnails.high.url.clone(),
                            alt: "",
                        }
                        div { class: "min-w-0",
                            div {
                                class: "truncate text-sm font-medium",
                                dangerous_inner_html: item.snippet.title.clone(),
                            }
                            div {
                                class: "truncate text-xs text-base-content/60",
                                dangerous_inner_html: item.snippet.channel_title.clone(),
                            }
                        }
                    }
                }

                if let Some(alert_props) = error() {
                    Alert { ..alert_props }
                }

                form { class: "flex gap-2 py-2", onsubmit: create_playlist,
                    div { class: "flex-1",
                        TextInput {
                            name: "playlist_name",
                            r#type: "text",
                            placeholder: "New playlist name",
                            Icon { icon: LdPlus }
                        }
                    }
                    button { class: "btn btn-primary self-center", r#type: "submit", "Create" }
                }

                if playlist_rows.read().is_empty() {
                    p { class: "py-4 text-center text-sm text-base-content/60",
                        "No playlists yet"
                    }
                } else {
                    ul { class: "list bg-base-100 rounded-box",
                        for entry in playlist_rows.read().iter() {
                            PlaylistOption {
                                playlist_id: entry.playlist.playlist_id,
                                name: entry.playlist.name.clone(),
                                thumbnails: entry.thumbnails(),
                                count: entry.tracks.len(),
                            }
                        }
                    }
                }

                div { class: "modal-action",
                    form { method: "dialog",
                        button { class: "btn", "Close" }
                    }
                }
            }
            form { method: "dialog", class: "modal-backdrop",
                button { "close" }
            }
        }
    }
}

#[component]
fn PlaylistOption(
    playlist_id: i32,
    name: String,
    thumbnails: Vec<String>,
    count: usize,
) -> Element {
    let playlists = use_playlists();

    rsx! {
        li {
            class: "list-row cursor-pointer transition-colors hover:bg-base-200/60",
            onclick: move |_| playlists.add_pending(playlist_id),
            div { class: "flex-shrink-0",
                CoverCollage { thumbnails, class: "size-12 rounded-box" }
            }
            div { class: "min-w-0",
                div { class: "truncate text-sm font-medium", "{name}" }
                div { class: "text-xs text-base-content/60",
                    "{count} "
                    if count == 1 { "song" } else { "songs" }
                }
            }
        }
    }
}
