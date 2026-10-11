pub mod db;
pub mod favorites;
pub mod history;
pub mod playlist;
pub mod settings;
pub mod youtube;

use db::models::Track;
use yt::data_api::types::{Item, Snippet, Thumb, Thumbnails, VideoId};

pub fn track_to_item(track: &Track) -> Item {
    Item {
        id: VideoId::Literal(track.track_id.clone()),
        snippet: Snippet {
            title: track.title.clone(),
            channel_title: track.channel_name.clone(),
            thumbnails: Thumbnails {
                high: Thumb {
                    url: track.thumbnail_url.clone(),
                },
            },
        },
    }
}

pub fn tracks_to_items(tracks: &[Track]) -> Vec<Item> {
    tracks.iter().map(track_to_item).collect()
}
