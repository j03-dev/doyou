mod types;

use dioxus::prelude::*;
use types::{Item, Videos};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dotenv::dotenv().ok();

    #[cfg(not(feature = "desktop"))]
    dioxus::launch(App);

    #[cfg(feature = "desktop")]
    {
        dioxus::LaunchBuilder::new()
            .with_cfg(
                dioxus::desktop::Config::default()
                    .with_menu(None)
                    .with_window(
                        dioxus::desktop::WindowBuilder::new()
                            .with_maximized(true)
                            .with_title("doyou"),
                    ),
            )
            .launch(App);
    }
}

#[component]
fn App() -> Element {
    let mut is_loading = use_signal(|| false);
    let mut search_query = use_signal(|| String::new());
    let mut status_msg = use_signal(|| None::<String>);
    let mut theme = use_signal(|| "light".to_string());
    let mut playback = use_context_provider(|| Playback::new("audio"));

    use_effect(move || {
        let _ = document::eval(&format!(
            r#"
                document.documentElement.setAttribute('data-theme', '{}')
           "#,
            theme()
        ));
    });

    let search = move |_| async move {
        if search_query().is_empty() {
            status_msg.set(Some("Please enter a search query.".to_string()));
            return;
        }

        status_msg.set(None);
        is_loading.set(true);

        match api_search(search_query()).await {
            Ok(videos) => playback.playlist.set(videos.items),
            Err(err) => status_msg.set(Some(err.to_string())),
        };
        is_loading.set(false);
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "navbar bg-base-100 shadow-sm text-secondary",
            audio { id: "audio" }
            div { class: "flex-1",
                a { class: "btn btn-ghost text-secondary text-xl", "DoYou" }
            }
            div { class: "flex-none",
                label { class: "swap swap-rotate",
                    input {
                        r#type: "checkbox",
                        class: "theme-controller",
                        onclick: move |_| {
                            let new_theme = if theme() == "light" { "dark" } else { "light" };
                            theme.set(new_theme.to_string());
                        },
                    }
                    svg {
                        class: "swap-off h-10 w-10 fill-current",
                        xmlns: "http://wwww.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        path { d: "M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" }
                    }
                    svg {
                        class: "swap-on h-10 w-10 fill-current",
                        xmlns: "http://wwww.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        path { d: "M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" }
                    }
                }
            }
        }
        div { class: "m-2 pb-24",
            form { class: "flex flex-row justify-center gap-2", onsubmit: search,
                label { class: "input input-secondary",
                    svg {
                        class: "h-[1em] opacity-50",
                        xmlns: "http://wwww.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        g {
                            stroke_linejoin: "round",
                            stroke_linecap: "round",
                            fill: "none",
                            stroke: "currentColor",
                            circle { cx: "11", cy: "11", r: "8" }
                            path { d: "m21 21-4.3-4.3" }
                        }
                    }
                    input {
                        r#type: "search",
                        class: "grow",
                        placeholder: "Search",
                        oninput: move |e| search_query.set(e.value()),
                    }
                }
                button { class: "btn btn-secondary", r#type: "submit", "Search" }
            }
            if let Some(message) = status_msg() {
                div { role: "alert", class: "alert alert-error my-5",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-6 w-6 shrink-0 stroke-current",
                        fill: "none",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
                        }
                    }
                    span { {message} }
                }
            }
            if is_loading() {
                div { class: "flex h-screen justify-center items-center",
                    span { class: "loading loading-spinner text-secondary size-20" }
                }
            } else {
                ul { class: "list bg-base-100 rounded-box shadow-md",
                    for item in playback.playlist.read().clone() {
                        MusicCard { item }
                    }
                }
            }
        }
        div { class: "fixed bottom-0 left-0 w-full bg-base-200 text-secondary shadow-inner",
            if playback.playing.read().as_ref().is_some() {
                MusicPlayer {}
            }
        }
    }
}

#[component]
fn MusicCard(item: Item) -> Element {
    let mut favorite = use_signal(|| false);
    let mut is_loading = use_signal(|| false);
    let mut playback = use_context::<Playback>();

    let it = item.clone();

    let start = move |_| {
        let it = it.clone();
        spawn(async move {
            is_loading.set(true);
            match api_get_url(it.id.video_id.clone()).await {
                Ok(url) => {
                    playback.playing.set(Some(it));
                    playback.start(url);
                }
                Err(_) => todo!(),
            };
            is_loading.set(false);
        });
    };

    rsx! {
        li { class: "list-row",
            div { onclick: start,
                img {
                    class: "size-30 rounded-box",
                    src: item.snippet.thumbnails.medium.url,
                }
            }
            div {
                div { {item.snippet.title} }
                div { class: "text-xs uppercase font-semibold opacity-60",
                    {item.snippet.channel_title}
                }
                p { class: "list-col-wrap text-xs mt-2", {item.snippet.description} }
                if is_loading() {
                    span { class: "loading loading-dots loading-sm" }
                }
            }
            button {
                class: "btn btn-square btn-ghost",
                onclick: move |_| favorite.set(!favorite()),
                svg {
                    class: "size-[1.2em]",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    g {
                        stroke_linejoin: "round",
                        stroke_linecap: "round",
                        stroke_width: "2",
                        fill: "none",
                        stroke: "currentColor",
                        class: {
                            if favorite() {
                                "fill-red-500 stroke-red-5000"
                            } else {
                                "fill-transparent stroke-current"
                            }
                        },
                        path { d: "M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" }
                    }
                }
            }
        }
    }
}

#[component]
fn MusicPlayer() -> Element {
    let mut playback = use_context::<Playback>();

    rsx! {
        div { class: "container mx-auto p-4 flex flex-col md:flex-row items-center justify-between gap-4",
            div { class: "flex items-center gap-4 w-full md:w-1/3",
                img {
                    class: "w-16 h-16 rounded-md object-cover",
                    alt: "Thumbnail",
                    src: {
                        playback
                            .playing
                            .clone()
                            .read()
                            .as_ref()
                            .map(|i| i.snippet.thumbnails.medium.url.clone())
                            .unwrap_or("https://via.placeholder.com/64".to_string())
                    },
                }
                div { class: "min-w-0",
                    p { class: "font-bold text-lg truncate",
                        {
                            playback
                                .playing
                                .read()
                                .as_ref()
                                .map(|i| i.snippet.title.clone())
                                .unwrap_or("Unknown".to_string())
                        }
                    }
                    p { class: "text-sm truncate",
                        {
                            playback
                                .playing
                                .read()
                                .as_ref()
                                .map(|i| i.snippet.channel_title.clone())
                                .unwrap_or("Unknown".to_string())
                        }
                    }
                }
            }
            div { class: "flex item-center gap-2 justify-center",
                button { class: "btn btn-ghost btn-circle",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-6 w-6",
                        fill: "currentColor",
                        view_box: "0 0 24 24",
                        path { d: "M18 18V6l-8 6 8 6zM6 6h2v12H6V6z" }
                    }
                }
                button {
                    onclick: move |_| playback.toggle_play(),
                    class: "btn btn-ghost btn-cirle",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-6 w-6",
                        fill: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            d: {
                                if *playback.is_playing.read() {
                                    "M6 4h4v16H6zM14 4h4v16h-4z"
                                } else {
                                    "M5 3l14 9-14 9V3z"
                                }
                            },
                        }
                    }
                }
                button { class: "btn btn-ghost btn-circle",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-6 w-6",
                        fill: "currentColor",
                        view_box: "0 0 24 24",
                        path { d: "M6 6v12l8-6-8-6zM18 6h-2v12h2V6z" }
                    }
                }
            }
            div { class: "w-full md: w-1/3",
                progress {
                    class: "progress progress-netural w-full",
                    value: "40",
                    max: "100",
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Playback {
    id: Signal<String>,
    is_playing: Signal<bool>,
    playing: Signal<Option<Item>>,
    playlist: Signal<Vec<Item>>,
}

impl Playback {
    fn new(id: &str) -> Self {
        Self {
            id: Signal::new(id.to_string()),
            is_playing: Signal::new(false),
            playing: Signal::new(None),
            playlist: Signal::new(Vec::new()),
        }
    }

    fn start(&mut self, src: String) {
        let id = self.id.read().clone();
        spawn(async move {
            let _ = document::eval(&format!(
                r#"
                   let audio = document.getElementById('{id}')
                   if (audio) {{
                       audio.src = '{src}'
                       audio.play()
                   }}
               "#
            ));
        });
        self.is_playing.set(true);
    }

    fn play(&mut self) {
        let id = self.id.read().clone();
        spawn(async move {
            let _ = document::eval(&format!(
                r#"
                   let audio = document.getElementById('{id}')
                   if (audio) audio.play()
                "#
            ));
        });
        self.is_playing.set(true);
    }

    fn pause(&mut self) {
        let id = self.id.read().clone();
        spawn(async move {
            let _ = document::eval(&format!(
                r#"
                   let audio = document.getElementById('{id}')
                   if (audio) audio.pause()
                "#
            ));
        });
        self.is_playing.set(false);
    }

    fn toggle_play(&mut self) {
        if *self.is_playing.read() {
            self.pause();
        } else {
            self.play();
        }
    }
}

#[get("/api/search")]
async fn api_search(name: String) -> Result<Videos, ServerFnError> {
    let key = match std::env::var("GOOGLE_API_KEY") {
        Ok(k) => k,
        Err(err) => return Err(ServerFnError::new(err.to_string())),
    };
    let query = format!("search?part=snippet&q={name}&type=video&maxResults=10&key={key}");
    match reqwest::get(format!("https://www.googleapis.com/youtube/v3/{query}")).await {
        Ok(response) => {
            if response.status().is_client_error() || response.status().is_server_error() {
                return Err(ServerFnError::new(response.text().await.unwrap()));
            }
            return Ok(response.json().await.unwrap());
        }
        Err(err) => Err(ServerFnError::new(err.to_string())),
    }
}

#[get("/api/url")]
async fn api_get_url(video_id: String) -> Result<String, ServerFnError> {
    let url = format!("https://www.youtube.com/watch?v={video_id}");
    match std::process::Command::new("yt-dlp")
        .args(&["-f", "bestaudio", "--get-url", "--no-playlist", &url])
        .output()
    {
        Ok(output) => {
            let audio_url = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(audio_url)
        }
        Err(err) => Err(ServerFnError::new(err.to_string())),
    }
}
