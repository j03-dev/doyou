use dioxus::prelude::*;

use crate::core::db;
use crate::core::db::models::YoutubeTrack;
use crate::core::error::Error;

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
    pub tracks: Signal<Vec<YoutubeTrack>>,
    pub is_loading: Signal<bool>,
}

impl FavoritesContext {
    pub fn new() -> Self {
        Self {
            tracks: Signal::new(Vec::new()),
            is_loading: Signal::new(false),
        }
    }

    pub async fn fetch_all(&mut self) -> Result<(), Error> {
        self.is_loading.set(true);
        self.tracks.set(db::get_all_favorites().await?);
        self.is_loading.set(false);
        Ok(())
    }

    pub async fn add(&mut self, track: YoutubeTrack) -> Result<(), Error> {
        self.is_loading.set(true);
        db::add_to_favorite(track.clone()).await?;
        self.tracks.write().push(track);
        self.is_loading.set(false);
        Ok(())
    }

    pub async fn remove(&mut self, youtube_track_id: &str) -> Result<(), Error> {
        self.is_loading.set(true);
        db::remove_from_favorite(youtube_track_id).await?;
        self.tracks.write().retain(|t| t.id != youtube_track_id);
        self.is_loading.set(false);
        Ok(())
    }
}
