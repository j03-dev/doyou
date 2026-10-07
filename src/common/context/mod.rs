pub mod favorites;
pub mod home;
pub mod playback;
pub mod settings;

pub use favorites::FavoritesProvider;
pub use home::HomeProvider;
pub use playback::PlaybackProvider;
pub use settings::AppSettingsProvider;

pub use favorites::use_favorites;
pub use home::use_home;
pub use playback::use_playback;
pub use settings::use_settings;
