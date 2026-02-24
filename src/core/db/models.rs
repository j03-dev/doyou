use rusql_alchemy::prelude::*;

#[derive(Debug, Clone, PartialEq, Model, serde::Deserialize)]
pub struct YoutubeTrack {
    #[field(primary_key = true, size = 11)]
    pub id: String,
    #[field(size = 255)]
    pub title: String,
    #[field(size = 100)]
    pub channel_name: String,
    #[field(size = 500)]
    pub thumbnail_url: String,
}

#[derive(Debug, Clone, Model, serde::Deserialize)]
pub struct Favorite {
    #[field(primary_key = true, auto = true)]
    pub fav_id: Option<Integer>,
    #[field(foreign_key = YoutubeTrack.id, on_delete = "cascade",unique = true, size = 11)]
    pub youtube_track_id: String,
}

#[derive(Debug, Clone, Model, serde::Deserialize)]
pub struct AppSettings {
    #[field(primary_key = true)]
    pub id: Integer, // always = 0
    #[field(size = 255)]
    pub youtube_token: String,
    #[field(size = 50, default = "Lofi")]
    pub theme: String,
}
