use dioxus::prelude::*;
use yt::data_api::types::{Item, Snippet, Thumb, Thumbnails, VideoId};

use crate::context::AlertProps;
use crate::repository;
use crate::repository::db::models::Track;

#[component]
pub fn FavoritesProvider(children: Element) -> Element {
    use_context_provider(FavoritesContext::new);
    rsx! {
        {children}
    }
}

pub fn use_favorites() -> FavoritesContext {
    use_context::<FavoritesContext>()
}

#[derive(Clone, Copy, PartialEq)]
pub struct FavoritesContext {
    pub tracks: Signal<Vec<Track>>,
    pub is_loading: Signal<bool>,
    pub error: Signal<Option<AlertProps>>,
}

impl FavoritesContext {
    pub fn new() -> Self {
        Self {
            tracks: Signal::new(Vec::new()),
            is_loading: Signal::new(false),
            error: Signal::new(None),
        }
    }

    pub fn fetch_all(&self) {
        let mut is_loading = self.is_loading;
        let mut error = self.error;
        let mut tracks = self.tracks;

        is_loading.set(true);
        error.set(None);

        spawn(async move {
            match repository::favorites::all().await {
                Ok(fetched) => tracks.set(fetched),
                Err(err) => error.set(Some(AlertProps::error(err))),
            }
            is_loading.set(false);
        });
    }

    pub fn toggle(&self, item: &Item) {
        let item = item.clone();
        let mut is_loading = self.is_loading;
        let mut error = self.error;
        let mut tracks = self.tracks;

        let Some(item_id) = item.id.as_string() else {
            return;
        };

        spawn(async move {
            is_loading.set(true);
            error.set(None);

            let is_fav = tracks.read().iter().any(|t| t.track_id == item_id);
            if is_fav {
                match repository::favorites::remove(&item_id).await {
                    Ok(()) => tracks.write().retain(|t| t.track_id != item_id),
                    Err(err) => error.set(Some(AlertProps::error(err))),
                }
            } else {
                let track = Track {
                    track_id: item_id,
                    title: item.snippet.title.clone(),
                    channel_name: item.snippet.channel_title.clone(),
                    thumbnail_url: item.snippet.thumbnails.high.url.clone(),
                    ..Default::default()
                };
                match repository::favorites::add(track).await {
                    Ok(new_track) => tracks.write().push(new_track),
                    Err(err) => error.set(Some(AlertProps::error(err))),
                }
            }
            is_loading.set(false);
        });
    }

    pub fn items(&self) -> Vec<Item> {
        self.tracks
            .read()
            .iter()
            .map(|t| Item {
                id: VideoId::Literal(t.track_id.clone()),
                snippet: Snippet {
                    title: t.title.clone(),
                    channel_title: t.channel_name.clone(),
                    thumbnails: Thumbnails {
                        high: Thumb {
                            url: t.thumbnail_url.clone(),
                        },
                    },
                },
            })
            .collect()
    }
}
