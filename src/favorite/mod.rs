use dioxus::prelude::*;
use yt::data_api::types::Item;
use yt::data_api::types::Snippet;
use yt::data_api::types::Thumb;
use yt::data_api::types::Thumbnails;
use yt::data_api::types::VideoId;

use crate::common::components::alert_message::AlertMessage;
use crate::common::components::music_list::MusicList;
use crate::common::context::{use_alert, use_favorites};

#[component]
pub fn Favorite() -> Element {
    let favorites = use_favorites();
    let mut alert = use_alert();
    let mut favorite_list = use_signal(|| Vec::new());

    use_effect(move || {
        favorites.fetch_all();
    });

    use_effect(move || {
        if let Some(err_msg) = favorites.error.read().as_ref() {
            alert.message.set(Some(err_msg.clone()));
        }
    });

    use_effect(move || {
        let items: Vec<Item> = favorites
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
            .collect();
        favorite_list.set(items);
    });

    rsx! {
        div { class: "m-5",
            if let Some(message) = &*alert.message.read() {
                AlertMessage { message: message.clone() }
            }
            MusicList { items: favorite_list() }
        }
    }
}
