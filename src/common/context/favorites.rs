use dioxus::prelude::*;

use crate::core::db;
use crate::core::db::models::YoutubeTrack;

#[derive(Clone, Copy)]
pub struct FavoritesContext {
    pub tracks: Signal<Vec<YoutubeTrack>>,
    pub is_loading: Signal<bool>,
    pub error: Signal<Option<String>>,
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
            match db::get_all_favorites().await {
                Ok(favs) => {
                    tracks.set(favs);
                }
                Err(e) => {
                    error.set(Some(e.to_string()));
                }
            };
            is_loading.set(false);
        });
    }

    pub fn add(&self, track: YoutubeTrack) {
        let mut is_loading = self.is_loading;
        let mut error = self.error;
        let mut tracks = self.tracks;

        is_loading.set(true);
        error.set(None);

        spawn(async move {
            match db::add_to_favorite(track.clone()).await {
                Ok(()) => {
                    tracks.write().push(track);
                }
                Err(e) => {
                    error.set(Some(e.to_string()));
                }
            };
            is_loading.set(false);
        });
    }

    pub fn remove(&self, youtube_track_id: &str) {
        let mut is_loading = self.is_loading;
        let mut error = self.error;
        let mut tracks = self.tracks;
        let track_id = youtube_track_id.to_string();

        is_loading.set(true);
        error.set(None);

        spawn(async move {
            match db::remove_from_favorite(&track_id).await {
                Ok(()) => {
                    tracks.write().retain(|t| t.id != track_id);
                }
                Err(e) => {
                    error.set(Some(e.to_string()));
                }
            };
            is_loading.set(false);
        });
    }
}

pub fn use_favorites() -> FavoritesContext {
    use_context::<FavoritesContext>()
}

#[component]
pub fn FavoritesProvider(children: Element) -> Element {
    let _favorites = use_context_provider(FavoritesContext::new);
    rsx! {
        {children}
    }
}
