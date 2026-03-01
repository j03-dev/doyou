use app::App;

mod app;
mod common;
mod core;
mod pages;
mod route;

fn main() {
    #[cfg(not(feature = "desktop"))]
    dioxus::launch(App);

    #[cfg(feature = "desktop")]
    {
        dioxus::LaunchBuilder::new()
            .with_cfg(
                dioxus::desktop::Config::default()
                    .with_menu(None)
                    .with_window(dioxus::desktop::WindowBuilder::new().with_title("doyou")),
            )
            .launch(App);
    }
}
