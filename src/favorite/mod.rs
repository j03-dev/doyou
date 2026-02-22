use dioxus::prelude::*;

use crate::core::db;

#[component]
pub fn Favorite() -> Element {
    let mut status_msg = use_context::<Signal<Option<String>>>();
    let mut favorites = use_signal(|| Vec::new());

    use_effect(move || {
        spawn(async move {
            match db::get_all_favorites().await {
                Ok(favs) => favorites.set(favs),
                Err(err) => status_msg.set(Some(err.to_string())),
            }
        });
    });

    rsx!(
        if favorites.is_empty() {
            p { "Favorites" }
        } else {
            for fav in favorites() {}
        }
    )
}
