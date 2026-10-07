use dioxus::prelude::*;
use yt::data_api::types::Item;

use crate::components::music_list::MusicList;
use crate::context::use_favorites;

#[component]
pub fn Favorite() -> Element {
    let favorites = use_favorites();

    use_effect(move || {
        favorites.fetch_all();
    });

    let items: Memo<Vec<Item>> = use_memo(move || favorites.items());

    rsx! {
        div { class: "m-5",
            MusicList { items: items() }
        }
    }
}
