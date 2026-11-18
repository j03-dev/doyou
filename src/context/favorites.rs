use dioxus::prelude::*;
use yt::data_api::types::Item;

use crate::context::AlertProps;
use crate::repository;
use crate::repository::db::models::Track;
use crate::repository::identity;

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

        if item.id.as_string().is_none() {
            return;
        }
        let is_fav = self.is_favorite_item(&item);

        spawn(async move {
            is_loading.set(true);
            error.set(None);

            if is_fav {
                match repository::favorites::remove(&item).await {
                    Ok(()) => tracks
                        .write()
                        .retain(|track| !identity::same_media(track, &item)),
                    Err(err) => error.set(Some(AlertProps::error(err))),
                }
            } else {
                match repository::favorites::add(&item).await {
                    Ok(new_track) => tracks.write().push(new_track),
                    Err(err) => error.set(Some(AlertProps::error(err))),
                }
            }
            is_loading.set(false);
        });
    }

    pub fn is_favorite_item(&self, item: &Item) -> bool {
        self.tracks
            .read()
            .iter()
            .any(|track| identity::same_media(track, item))
    }

    pub fn items(&self) -> Vec<Item> {
        repository::tracks_to_items(&self.tracks.read())
    }
}
