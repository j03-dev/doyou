use dioxus::prelude::*;

use crate::core::db::models::YoutubeTrack;

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
}

pub fn use_favorites() -> FavoritesContext {
    use_context::<FavoritesContext>()
}

#[component]
pub fn FavoritesProvider(children: Element) -> Element {
    let _favorites = use_context_provider(FavoritesContext::new);
    rsx! { {children} }
}
