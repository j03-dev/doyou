mod app;
mod components;
mod providers;
mod servers;

fn main() {
    dotenv::dotenv().ok();

    #[cfg(not(feature = "desktop"))]
    dioxus::launch(app::App);

    #[cfg(feature = "desktop")]
    {
        dioxus::LaunchBuilder::new()
            .with_cfg(
                dioxus::desktop::Config::default()
                    .with_menu(None)
                    .with_window(dioxus::desktop::WindowBuilder::new().with_title("doyou")),
            )
            .launch(app::App);
    }
}
