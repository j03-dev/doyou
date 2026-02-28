use dioxus::prelude::*;
use yt::data_api::types::Item;
use yt::data_api::types::Snippet;
use yt::data_api::types::Thumb;
use yt::data_api::types::Thumbnails;
use yt::data_api::types::VideoId;

use crate::common::components::music_list::MusicList;
use crate::common::context::use_favorites;

#[component]
pub fn Favorite() -> Element {
    let favorites = use_favorites();

    use_effect(move || {
        favorites.fetch_all();
    });

    let items = use_memo(move || {
        favorites
            .tracks
            .read()
            .iter()
            .map(|t| Item {
                id: VideoId::Literal(t.id.clone()),
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
            .collect::<Vec<Item>>()
    });

    rsx! {
        div { class: "m-5",
            MusicList { items: items.read().clone()}
        }
    }
}
