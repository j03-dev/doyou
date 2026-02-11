mod components;
mod config;
mod providers;

fn main() {
    #[cfg(not(feature = "desktop"))]
    dioxus::launch(components::App);

    #[cfg(feature = "desktop")]
    {
        dioxus::LaunchBuilder::new()
            .with_cfg(
                dioxus::desktop::Config::default()
                    .with_menu(None)
                    .with_window(dioxus::desktop::WindowBuilder::new().with_title("doyou")),
            )
            .launch(components::App);
    }
}
