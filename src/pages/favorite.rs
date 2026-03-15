use dioxus::prelude::*;
use yt::data_api::types::{Item, Snippet, Thumb, Thumbnails, VideoId};

use crate::common::components::alert::{Alert, AlertProps};
use crate::common::components::music_list::MusicList;
use crate::common::context::use_favorites;

#[component]
pub fn Favorite() -> Element {
    let mut favorites = use_favorites();
    let mut alert = use_signal(|| None::<AlertProps>);

    use_effect(move || {
        spawn(async move {
            alert.set(None);
            if let Err(err) = favorites.fetch_all().await {
                alert.set(Some(AlertProps::error(err.to_string())));
            }
        });
    });

    let items: Memo<Vec<Item>> = use_memo(move || {
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
            .collect()
    });

    rsx! {
        div { class: "m-5",
            if let Some(alert_props) = alert() {
                Alert { ..alert_props }
            }
            MusicList { items: items() }
        }
    }
}
