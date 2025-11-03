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
    let (status_msg, set_status_msg) = signal(Option::None);

    let (isloading, set_isloading) = signal(false);

    let update_query = move |ev| {
        let v = event_target_value(&ev);
        set_search_query.set(v);
    };

    let search_videos = move |ev: SubmitEvent| {
        ev.prevent_default();

        spawn_local(async move {
            let query = search_query.get_untracked();
            if query.is_empty() {
                set_status_msg.set(Some("Please entrer a search query.".to_string()));
                return;
            }
            
            set_isloading.set(true);
            set_status_msg.set(None);
            
			let args = serde_wasm_bindgen::to_value(&SearchArgs { name: &query }).unwrap();		
            let js_value = invoke("search", args).await;

            match serde_wasm_bindgen::from_value::<Response<Videos, String>>(js_value) {
                Ok(Response::Success(videos)) => set_videos.set(videos.items),
                Ok(Response::Failed(err)) => {
                    set_status_msg.set(Some(format!("Search failed: {err}")))
                }
                Err(err) => set_status_msg.set(Some(format!("failed to parse response {err}"))),
            };
            set_isloading.set(false);
        });
    };

    view! {
            <main>
				<div class="navbar bg-base-100 shadow-sm text-neutral">
				  <div class="flex-1">
					<a class="btn btn-ghost text-neutral text-xl">daisyUI</a>
				  </div>
				  <div class="flex-none">
					<label class="swap swap-rotate">
					  <input type="checkbox" class="theme-controller" value="synthwave" />
					  <svg
						class="swap-off h-10 w-10 fill-current"
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 24 24">
						<path
						  d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" />
					  </svg>
					  <svg
						class="swap-on h-10 w-10 fill-current"
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 24 24">
						<path
						  d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" />
					  </svg>
					</label>
				  </div>
			    </div>
 
                <div class="m-2">
                    <form class="flex flex-row justify-center gap-2" on:submit=search_videos>
                       <label class="input input-neutral">
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
                      <button class="btn btn-neutral" type="submit">"Search"</button>
                    </form>

                    <Show when=move || status_msg.get().is_some()>
                        <div role="alert" class="alert alert-error my-5">
                          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                          </svg>
                          <span>{ move || status_msg.get().unwrap() }</span>
                        </div>
                    </Show>

                    <Show
                        when=move || isloading.get()
                        fallback=move || view! {
                            <ul class="list bg-base-100 rounded-box shadow-md">
                                <For
                                    each=move || videos.get()
                                    key=|item| item.id.video_id.clone()
                                    children=move |item: Item| {
                                        view! {
                                            <li class="list-row">
                                                <div>
                                                    <img class="size-50 rounded-box" src={ move || item.snippet.thumbnails.medium.url.clone() } />
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
                        }
                    >
                        <div class="flex justify-center items-center">
                            <span class="loading loading-spinner text-neutral size-30"></span>
                        </div>
                    </Show>
                </div>
            </main>
        }
}
