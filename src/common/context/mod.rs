pub mod favorites;
pub mod playback;
pub mod settings;

pub use favorites::FavoritesProvider;
pub use playback::PlaybackProvider;
pub use settings::AppSettingsProvider;

pub use favorites::use_favorites;
pub use playback::use_playback;
pub use settings::use_settings;
