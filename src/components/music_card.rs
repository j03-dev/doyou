use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};

use crate::invoke;
use crate::music_player::MusicPlayer;
use commons::{Download, Item, Response};

#[derive(Deserialize, Serialize)]
struct DownloadArgs<'a> {
    video_id: &'a str,
}

#[component]
pub fn MusicCard(item: Item) -> impl IntoView {
    let player = use_context::<MusicPlayer>().unwrap();
    let (is_loading, set_is_loading) = signal(false);
    let (favorite_state, set_favorite_state) = signal(false);
    let arc_item = std::sync::Arc::new(item.clone());

    let start_music = move |_| {
        let video_id = item.id.video_id.clone();
        let item = arc_item.clone();
        spawn_local(async move {
            set_is_loading.set(true);
            let args = serde_wasm_bindgen::to_value(&DownloadArgs {
                video_id: &video_id,
            })
            .unwrap();
            let js_value = invoke("download", args).await;
            match serde_wasm_bindgen::from_value::<Response<Download, String>>(js_value) {
                Ok(v) => match v {
                    Response::Success(downloaded) => {
                        set_is_loading.set(false);
                        player.playing.set(Some(item.as_ref().clone()));
                        player.start(&downloaded.url);
                    }
                    _ => set_is_loading.set(false),
                },
                _ => set_is_loading.set(false),
            }
        });
    };

    view! {
        <li class="list-row">
            <div on:click=start_music>
                <img class="size-50 rounded-box" src={ item.snippet.thumbnails.medium.url.clone() } />
            </div>
            <div>
                <div> { item.snippet.title.clone() } </div>
                <div class="text-xs uppercase font-semibold opacite-60">
                    { item.snippet.channel_title.clone() }
                </div>
                <p class="list-col-wrap text-xs"> { item.snippet.description.clone() } </p>

                <button class="btn btn-square btn-ghost" on:click=move |_| set_favorite_state.set(!favorite_state.get())>
                    <svg class="size-[1.2em]" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                        <g stroke-linejoin="round"
                           stroke-linecap="round"
                           stroke-width="2"
                           fill="none"
                           stroke="currentColor"
                           class={ move || {
                               if favorite_state.get() {"fill-red-500 stroke-red-500"}
                               else {"fill-transparent stroke-current"}
                            }}>
                            <path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z">
                            </path>
                        </g>
                    </svg>
                </button>

                <Show when=move || is_loading.get()>
                    <span class="loading loading-dots loading-sm"></span>
                 </Show>
            </div>
       </li>
    }
}
