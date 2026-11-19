pub mod alert;
pub mod downloads;
pub mod favorites;
pub mod history;
pub mod home;
pub mod playback;
pub mod playlist;
pub mod settings;

pub use alert::{AlertProps, AlertProvider, offline_alert, use_alerts};
pub use downloads::DownloadsProvider;
pub use favorites::FavoritesProvider;
pub use history::HistoryProvider;
pub use home::{HomeMode, HomeProvider, SourceFilter};
pub use playback::PlaybackProvider;
pub use playlist::PlaylistProvider;
pub use settings::AppSettingsProvider;

pub use downloads::use_downloads;
pub use favorites::use_favorites;
pub use history::use_history;
pub use home::use_home;
pub use playback::use_playback;
pub use playlist::use_playlists;
pub use settings::use_settings;
