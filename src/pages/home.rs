use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdKey, LdMenu, LdPlus, LdSearch, LdX};

use crate::components::alert::Alert;
use crate::components::button::ButtonGhost;
use crate::components::form::get_value_from;
use crate::components::loading::LoadingSpinner;
use crate::components::music_card::MusicCard;
use crate::components::navbar::{NavBar, NavBarItem, NavBarPos};
use crate::components::playlist_card::PlaylistCard;
use crate::components::text_input::TextInput;
use crate::context::{
    HomeMode, use_favorites, use_home, use_playback, use_playlists, use_settings,
};
use crate::route::Route;

#[component]
pub fn Home() -> Element {
    let settings = use_settings();
    let home = use_home();
    let playback = use_playback();
    let favorites = use_favorites();
    let playlists = use_playlists();

    let mode = home.mode;
    let home_error = home.error;
    let home_is_loading = home.is_loading;
    let settings_error = settings.error;
    let playlist_error = playlists.error;

    use_effect(move || {
        if settings.general.read().youtube_token.is_none() {
            document::eval("token_form.showModal()");
        }
    });

    use_effect(move || {
        home.load_feed();
    });

    use_effect(move || {
        favorites.fetch_all();
    });

    use_effect(move || {
        playlists.fetch_all();
    });

    let displayed: Memo<Vec<yt::data_api::types::Item>> = use_memo(move || match mode() {
        HomeMode::Results => home.results.read().clone(),
        _ => home.feed.read().clone(),
    });

    use_effect(move || {
        playback.set_queue(displayed());
    });

    let favorite_thumbnails = use_memo(move || {
        favorites
            .tracks
            .read()
            .iter()
            .take(4)
            .map(|t| t.thumbnail_url.clone())
            .collect::<Vec<_>>()
    });
    let favorite_count = use_memo(move || favorites.tracks.read().len());

    let search = move |evt: Event<FormData>| {
        evt.prevent_default();
        let search_query = get_value_from(evt, "search").unwrap_or_default();
        home.search(search_query);
    };

    let submit_token = move |evt: Event<FormData>| {
        evt.prevent_default();
        let Some(token) = get_value_from(evt, "token") else {
            return;
        };
        if token.trim().is_empty() {
            settings.save_token(token);
            return;
        }
        settings.save_token(token);
        spawn(async move {
            let _ = document::eval("token_form.close()").await;
        });
    };

    let submit_playlist = move |evt: Event<FormData>| {
        evt.prevent_default();
        let name = get_value_from(evt, "playlist_name").unwrap_or_default();
        playlists.create(name);
    };

    rsx! {
        NavBar {
            NavBarItem { position: NavBarPos::Start, ThemeController {} }
            NavBarItem { position: NavBarPos::Center,
                if mode() == HomeMode::Feed {
                    p { class: "btn btn-ghost text-xl", "DoYou" }
                } else {
                    form { onsubmit: search,
                        TextInput {
                            name: "search",
                            r#type: "search",
                            placeholder: "Search",
                            Icon { icon: LdSearch, fill: "grey", width: 16 }
                        }
                    }
                }
            }
            NavBarItem { position: NavBarPos::End,
                if mode() == HomeMode::Feed {
                    ButtonGhost { onclick: move |_| home.open_search(),
                        Icon { icon: LdSearch }
                    }
                } else {
                    ButtonGhost { onclick: move |_| home.close_search(),
                        Icon { icon: LdX }
                    }
                }
            }
        }

        div { class: format!(
                "px-2 pt-2 mx-auto w-full max-w-6xl {}",
                if playback.playing.read().is_some() { "pb-48" } else { "pb-28" }
            ),
            if let Some(alert_props) = home_error() {
                Alert { ..alert_props }
            }
            if let Some(alert_props) = settings_error() {
                Alert { ..alert_props }
            }
            if let Some(alert_props) = playlist_error() {
                Alert { ..alert_props }
            }

            if mode() == HomeMode::Searching {
                div { class: "min-h-[60vh]" }
            } else if mode() == HomeMode::Results {
                if home_is_loading() {
                    div { class: "flex h-screen justify-center items-center",
                        LoadingSpinner { size: 20 }
                    }
                } else {
                    div { class: "grid grid-cols-2 gap-3 sm:grid-cols-3 lg:grid-cols-4 2xl:grid-cols-5",
                        for (index, item) in displayed().iter().enumerate() {
                            MusicCard { item: item.clone(), index }
                        }
                    }
                }
            } else if home.feed.read().is_empty() && home_is_loading() {
                div { class: "flex h-screen justify-center items-center",
                    LoadingSpinner { size: 20 }
                }
            } else {
                div { class: "flex items-center justify-between px-1 pt-4 pb-2",
                    h2 { class: "text-lg font-semibold", "Your Library" }
                    button {
                        class: "btn btn-sm btn-ghost gap-1",
                        onclick: move |_| playlists.open_create(),
                        Icon { icon: LdPlus, class: "size-4" }
                        "New playlist"
                    }
                }
                div { class: "carousel carousel-end gap-3 w-full",
                    PlaylistCard {
                        name: "Favorites".to_string(),
                        thumbnails: favorite_thumbnails(),
                        count: favorite_count(),
                        to: Route::Favorite {},
                    }
                    for entry in playlists.items.read().iter() {
                        PlaylistCard {
                            name: entry.playlist.name.clone(),
                            thumbnails: entry.thumbnails(),
                            count: entry.tracks.len(),
                            to: Route::Playlist { id: entry.playlist.playlist_id },
                        }
                    }
                    div { class: "carousel-item",
                        button {
                            class: "card w-36 shrink-0 border border-dashed border-base-content/30 bg-base-100 transition-colors hover:bg-base-200/60 sm:w-40",
                            onclick: move |_| playlists.open_create(),
                            div { class: "flex aspect-square items-center justify-center",
                                Icon { icon: LdPlus, class: "size-8 text-base-content/50" }
                            }
                            div { class: "px-1 pt-2 pb-1",
                                p { class: "text-sm font-semibold", "New playlist" }
                            }
                        }
                    }
                }

                h2 { class: "px-1 pt-6 pb-2 text-lg font-semibold", "Trending" }
                div { class: "carousel gap-3 w-full",
                    for (index, item) in displayed().iter().enumerate() {
                        div { class: "carousel-item w-40 sm:w-48",
                            MusicCard { item: item.clone(), index }
                        }
                    }
                }
            }
        }

        dialog { id: "create_playlist", class: "modal",
            div { class: "modal-box w-96 max-w-[calc(100vw-2rem)]",
                h3 { class: "font-bold text-lg", "New playlist" }
                form { onsubmit: submit_playlist,
                    fieldset { class: "fieldset",
                        legend { class: "fieldset-legend", "Playlist name" }
                        TextInput {
                            name: "playlist_name",
                            r#type: "text",
                            placeholder: "My playlist",
                        }
                    }
                    button { class: "btn btn-primary mt-3", r#type: "submit", "Create" }
                }
                div { class: "modal-action",
                    form { method: "dialog",
                        button { class: "btn", "Cancel" }
                    }
                }
            }
            form { method: "dialog", class: "modal-backdrop",
                button { "close" }
            }
        }

        dialog { id: "token_form", class: "modal",
            div { class: "modal-box w-96 max-w-[calc(100vw-2rem)]",
                form { method: "dialog",
                    button {
                        class: "btn btn-sm btn-circle btn-ghost absolute right-3 top-3",
                        "aria-label": "Close",
                        Icon { icon: LdX }
                    }
                }
                div { class: "flex items-center gap-3 pb-2",
                    div { class: "flex size-10 shrink-0 items-center justify-center rounded-lg bg-primary/10 text-primary",
                        Icon { icon: LdKey, class: "size-5" }
                    }
                    div { class: "min-w-0",
                        h3 { class: "text-lg font-bold", "Connect YouTube" }
                        p { class: "text-sm text-base-content/60",
                            "Add your API key to load music."
                        }
                    }
                }
                if let Some(alert_props) = settings_error() {
                    Alert { ..alert_props }
                }
                form { class: "flex flex-col gap-3 pt-2", onsubmit: submit_token,
                    fieldset { class: "fieldset",
                        legend { class: "fieldset-legend", "YouTube Data API key" }
                        TextInput {
                            name: "token",
                            r#type: "password",
                            placeholder: "AIzaSy...",
                            required: true,
                            autofocus: true,
                            Icon { icon: LdKey, class: "size-5 opacity-60" }
                        }
                        p { class: "label gap-1",
                            "Create a key in the "
                            a {
                                class: "link link-primary",
                                href: "https://console.cloud.google.com/apis/credentials",
                                target: "_blank",
                                "Google Cloud Console"
                            }
                        }
                    }
                    button { class: "btn btn-primary w-full", r#type: "submit", "Save & Continue" }
                }
            }
            form { method: "dialog", class: "modal-backdrop",
                button { "close" }
            }
        }

    }
}

#[component]
fn ThemeController() -> Element {
    let settings = use_settings();
    let themes = &["Lofi", "Black", "Nord", "Pastel", "Slik", "Sunset"];

    rsx! {
        div { class: "dropdown",
            div {
                tabindex: 0,
                role: "button",
                class: "btn btn-ghost btn-circle",
                Icon { icon: LdMenu }
            }
            ul {
                tabindex: -1,
                class: "dropdown-content bg-base-300 rounded-box z-1 w-52 p-2 shadow-2xl",
                for theme in themes {
                    ThemeItem {
                        name: theme,
                        callback: move |theme| {
                            settings.save_theme(theme);
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn ThemeItem(name: &'static str, callback: Callback<String>) -> Element {
    rsx! {
        li {
            input {
                r#type: "radio",
                name: "theme-dropdown",
                class: "theme-controller w-full btn btn-sm btn-block btn-ghost justify-start",
                aria_label: name,
                value: name.to_lowercase(),
                onclick: move |_| callback.call(name.to_lowercase()),
            }
        }
    }
}
