use dioxus::prelude::*;

use crate::core::db;
use crate::core::db::models::YoutubeTrack;
use crate::core::error::Error;

#[derive(Clone)]
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
        let tracks = db::get_all_favorites().await?;
        self.tracks.set(tracks);
        Ok(())
    }

    pub async fn add(&mut self, track: YoutubeTrack) -> Result<(), Error> {
        db::add_to_favorite(track.clone()).await?;
        self.tracks.write().push(track);
        Ok(())
    }

    pub async fn remove(&mut self, youtube_track_id: &str) -> Result<(), Error> {
        db::remove_from_favorite(youtube_track_id).await?;
        self.tracks.write().retain(|t| t.id != youtube_track_id);
        Ok(())
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
