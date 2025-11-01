use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use common::{Item, Response, Videos};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Deserialize, Serialize)]
struct SearchArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());
    let (videos, set_videos) = signal(Vec::<Item>::new());
    let (status_msg, set_status_msg) = signal(String::new());

    let update_query = move |ev| {
        let v = event_target_value(&ev);
        set_search_query.set(v);
    };

    let search_videos = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_status_msg.set("Searching...".to_string());

        spawn_local(async move {
            let query = search_query.get_untracked();
            if query.is_empty() {
                set_status_msg.set("Please entrer a search query.".to_string());
            }
            let args = serde_wasm_bindgen::to_value(&SearchArgs { name: &query }).unwrap();

            let js_value = invoke("search", args).await;

            match serde_wasm_bindgen::from_value::<Response<Videos, String>>(js_value) {
                Ok(Response::Success(videos)) => set_videos.set(videos.items),
                Ok(Response::Failed(e)) => set_status_msg.set(format!("Search failed: {e}")),
                Err(err) => panic!("{err}"),
            };
        });
    };

    view! {
        <main class="m-2">
			<form class="flex flex-row center-item justify-center gap-2" on:submit=search_videos>
			   <label class="input">
				  <svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
					<g
					  stroke-linejoin="round"
					  stroke-linecap="round"
					  stroke-width="2.5"
					  fill="none"
					  stroke="currentColor"
					>
					  <circle cx="11" cy="11" r="8"></circle>
					  <path d="m21 21-4.3-4.3"></path>
					</g>
				  </svg>
				  <input type="search" class="grow" placeholder="Search" on:input=update_query />
			  </label>
			  <button class="btn" type="submit">"Search"</button>
			</form>
            
           <ul class="list bg-base-100 rounded-box shadow-md">
           <For
                each=move || videos.get()
                key=|item| item.id.video_id.clone()
                children=move |item: Item| {
					view! {
						<li class="list-row">
							<div>
								<img
									class="size-50 rounded-box"
									src={ move || item.snippet.thumbnails.medium.url.clone()  }
								/>
							</div>
							<div>
								<div> { move || item.snippet.title.clone() } </div>
								<div class="text-xs uppercase font-semibold opacite-60">
									{ move || item.snippet.channel_title.clone() }
								</div>
							    <p class="list-col-wrap text-xs"> { move || item.snippet.description.clone() } </p>
							    <button class="btn btn-square btn-ghost">
									<svg class="size-[1.2em]" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor"><path d="M6 3L20 12 6 21 6 3z"></path></g></svg>
								</button>
								<button class="btn btn-square btn-ghost">
								  <svg class="size-[1.2em]" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor"><path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"></path></g></svg>
								</button>
							</div>
						</li>
					}
                }
           />
           </ul>

        </main>
    }
}
