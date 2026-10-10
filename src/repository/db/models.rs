use rusql_alchemy::prelude::*;

#[derive(Debug, Clone, PartialEq, Model, serde::Deserialize)]
pub struct Track {
    #[field(primary_key = true, size = 11)]
    pub track_id: String,

    #[field(size = 255)]
    pub title: String,

    #[field(size = 100)]
    pub channel_name: String,

    #[field(size = 500)]
    pub thumbnail_url: String,

    pub src: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Model, serde::Deserialize)]
pub struct TrackPlaylist {
    #[field(primary_key = true, auto = true)]
    pub track_playlist_id: Option<Integer>,

    #[field(foreign_key = Track.track_id, on_delete = "cascade")]
    pub track_playlist_fk_track_id: String,

    #[field(foreign_key = Playlist.playlist_id, on_delete = "cascade")]
    pub track_playlist_fk_playlist_id: Integer,
}

#[derive(Debug, Clone, PartialEq, Model, serde::Deserialize)]
pub struct Playlist {
    #[field(primary_key = true, auto = true)]
    pub playlist_id: Integer,

    pub name: String,
}

#[derive(Debug, Clone, Model, serde::Deserialize)]
pub struct Favorite {
    #[field(primary_key = true, auto = true)]
    pub favorite_id: Option<Integer>,

    #[field(foreign_key = Track.track_id, on_delete = "cascade", unique = true, size = 11)]
    pub favorite_fk_track_id: String,
}

#[derive(Debug, Clone, Model, serde::Deserialize)]
pub struct History {
    #[field(primary_key = true, auto = true)]
    pub history_id: Option<Integer>,

    #[field(foreign_key = Track.track_id, on_delete = "cascade", unique = true, size = 11)]
    pub history_fk_track_id: String,

    #[field(default = 0)]
    pub play_count: Integer,

    #[field(default = 0)]
    pub last_played_at: Float,
}

#[derive(Debug, Clone, Model, serde::Deserialize)]
pub struct AppSettings {
    #[field(primary_key = true)]
    pub id: Integer,

    #[field(size = 255)]
    pub youtube_token: Option<String>,

    #[field(size = 50, default = "Lofi")]
    pub theme: String,
}
