use dioxus::prelude::*;

use crate::playback::Playback;

#[component]
pub fn MusicPlayer() -> Element {
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
                // prev button
                button {
                    onclick: move |_| playback.playback_controller(-1),
                    class: "btn btn-ghost btn-circle",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-6 w-6",
                        fill: "currentColor",
                        view_box: "0 0 24 24",
                        path { d: "M18 18V6l-8 6 8 6zM6 6h2v12H6V6z" }
                    }
                }
                // play button
                if !*playback.is_loading.read() {
                    button {
                        class: "btn btn-circle btn-primary",
                        onclick: move |_| playback.toggle_play(),
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
                } else {
                    button { class: "btn btn-circle btn-primary",
                        span { class: "loading loading-spinner" }
                    }
                }
                // next button
                button {
                    onclick: move |_| playback.playback_controller(1),
                    class: "btn btn-ghost btn-circle",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-6 w-6",
                        fill: "currentColor",
                        view_box: "0 0 24 24",
                        path { d: "M6 6v12l8-6-8-6zM18 6h-2v12h2V6z" }
                    }
                }
            }
            div { class: "w-full md:w-1/3",
                progress {
                    class: "progress progress-primary w-full",
                    value: playback.current_time.read().to_string(),
                    max: playback.duration.read().to_string(),
                }
            }
        }
    }
}
