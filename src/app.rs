use dioxus::prelude::*;

use crate::components::*;
use crate::providers::Playback;
use crate::servers;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    let mut is_loading = use_signal(|| false);
    let mut search_query = use_signal(|| String::new());
    let mut status_msg = use_signal(|| None::<String>);
    let mut playback = use_context_provider(|| Playback::new("audio"));

    use_effect(move || {
        spawn(async move {
            match servers::api_suggestion().await {
                Ok(videos) => playback.playlist.set(videos.items),
                Err(err) => status_msg.set(Some(err.to_string())),
            };
        });
    });

    let search = move |evt: Event<FormData>| async move {
        evt.prevent_default();
        if search_query().is_empty() {
            status_msg.set(Some("Please enter a search query.".to_string()));
            return;
        }

        status_msg.set(None);
        is_loading.set(true);

        match servers::api_search(search_query()).await {
            Ok(videos) => playback.playlist.set(videos.items),
            Err(err) => status_msg.set(Some(err.to_string())),
        };
        is_loading.set(false);
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        NavBar {}
        div { class: "m-2 pb-24",
            form { class: "flex flex-row justify-center gap-2", onsubmit: search,
                label { class: "input input-primary",
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
                ul { class: "list bg-base-100 rounded-box shadow-md pt-5",
                    for (index , item) in playback.playlist.read().iter().enumerate() {
                        MusicCard { item: item.clone(), index }
                    }
                }
            }
        }
        div { class: "fixed bottom-0 left-0 w-full bg-base-200 shadow-inner",
            audio {
                id: playback.id,
                onended: move |_| playback.playback_controller(1),
                ontimeupdate: move |_| playback.update_current_time(),
                ondurationchange: move |_| playback.update_duration(),
            }
            if playback.playing.read().as_ref().is_some() {
                MusicPlayer {}
            }
        }
    }
}
