use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

use common::{Response, Videos, Item};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}


#[derive(Deserialize, Serialize)]
struct SearchArgs<'a> {
	name: &'a str
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
	
	let search_videos = move |ev: SubmitEvent|{
		ev.prevent_default();
		set_status_msg.set("Searching...".to_string());
		
		spawn_local(async move {
			let query = search_query.get_untracked();
			if query.is_empty() {
				set_status_msg.set("Please entrer a search query.".to_string());
			}
			let args = serde_wasm_bindgen::to_value(&SearchArgs {name: &query}).unwrap();
			
			let js_value = invoke("search", args).await;
			
		    match serde_wasm_bindgen::from_value::<Response<Videos, String>>(js_value) {
                Ok(Response::Success(videos)) => set_videos.set(videos.items),
                Ok(Response::Failed(e)) => set_status_msg.set(format!("Search failed: {e}")),
                Err(err) => panic!("{err}"),
            };	
		});
	};
		
	view! {
        <main class="container">
        
            <form class="row" on:submit=search_videos>
                <input
                    id="search-input"
                    placeholder="Enter video search query..."
                    on:input=update_query
                />
                <button type="submit">"Search"</button>
            </form>
            
            <p>{ move || status_msg.get() }</p>
            
           <For
				each=move || videos.get()
				key=|item| item.id.video_id.clone()
				children=move |item: Item| {
					view! {
						<button>{move || item.snippet.title.clone() }</button>
						<button>{move || item.snippet.description.clone() }</button>
					}
				}
		   /> 

        </main>
    }
}
