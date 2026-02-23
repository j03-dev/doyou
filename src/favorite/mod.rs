use crate::common::components::alert_message::AlertMessage;
use crate::common::components::button::IconButton;
use crate::common::components::icons::FavoriteIcon;
use crate::core::db;
use crate::core::db::models::YoutubeTrack;
use dioxus::prelude::*;

#[component]
pub fn Favorite() -> Element {
    let mut status_msg = use_context_provider(|| Signal::new(None::<String>));
    let mut favorites: Signal<Vec<YoutubeTrack>> = use_signal(|| Vec::new());

    use_effect(move || {
        spawn(async move {
            match db::get_all_favorites().await {
                Ok(favs) => favorites.set(favs),
                Err(err) => status_msg.set(Some(err.to_string())),
            }
        });
    });

    rsx! {
        if let Some(message) = status_msg() {
            AlertMessage { message }
        }
        div { class: "grid grid-cols-1 md:grid-cols-4 gap-6 p-4 justify-items-center",
            for track in favorites() {
                {
                    let track_clone = track.clone();
                    rsx! {
                        div { class: "card card-sm bg-base-100 w-full max-w-sm shadow-sm",
                            figure { class: "px-5 pt-5",
                                img { src: track.thumbnail_url.clone(), class: "rounded-xl size-full" }
                            }
                            div { class: "card-body items-start text-left",
                                div {
                                    h2 { class: "card-title text-sm", "{track.title}" }
                                    p { class: "text-xs opacity-70", "{track.channel_name}" }
                                }
                                IconButton {
                                    on_click: move |_| {
                                        let track_id = track_clone.id.clone();
                                        spawn(async move {
                                            if let Err(err) = db::remove_from_favorite(&track_id).await {
                                                status_msg.set(Some(err.to_string()));
                                            } else {
                                                favorites.write().retain(|t| t.id != track_id);
                                            }
                                        });
                                    },
                                    FavoriteIcon { class: "fill-red-500 stroke-current-500" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
