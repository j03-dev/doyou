use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};

use crate::components::MusicCard;
use crate::services;
use crate::types::{Item, Response};
use crate::music_player::MusicPlayer;


#[component]
pub fn Player() -> impl IntoView {
	let music_player = MusicPlayer::new();
	provide_context(music_player);
	
	let item = Memo::new(move |_| music_player.item.get());
	
	view! {
		<Show when= move || item.get().is_some()>
			<div class="container mx-auto p-4 flex items-center justify-between">
				<audio node_ref=music_player.audio_ref on:ended=move |_| music_player.is_playing.set(false)/>
				<div class="flex items-center gap-4 w-1/3">
					<img src={move || item.get().map(|i| i.snippet.thumbnails.medium.url).unwrap_or("https://via.placeholder.com/64".to_string()) } alt="Thumbnail" class="w-16 h-16 rounded-md object-cover" />
					<div>
						<p class="font-bold text-lg">{move || item.get().map(|i| i.snippet.title).unwrap_or("Unknow title".to_string()) }</p>
						<p class="text-sm">{"Artist Name"}</p>
					</div>
				</div>
				<div class="flex items-center gap-2 justify-center flex-grow">
					<button class="btn btn-ghost btn-circle">
						<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24">
							<path d="M18 18V6l-8 6 8 6zM6 6h2v12H6V6z" />
						</svg>
					</button>
					<button class="btn btn-ghost btn-circle" on:click=move |_| {
						if music_player.is_playing.get() { music_player.pause() }
						else { music_player.play() }
					}>
						{move || if music_player.is_playing.get() {
							view! {
								<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24">
									<path d="M6 4h4v16H6zM14 4h4v16h-4z" />
								</svg>
							}
						} else {
							view! {
								<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24">
									<path d="M5 3l14 9-14 9V3z" />
								</svg>
							}
						}}
					</button>
					<button class="btn btn-ghost btn-circle">
						<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24">
							<path d="M6 6v12l8-6-8-6zM18 6h-2v12h2V6z" />
						</svg>
					</button>
				</div>
				<div class="w-1/3 text-right">
					<progress class="progress progress-neutral" value="40" max="100"></progress>
				</div>
			</div>
		</Show>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());
    let (videos, set_videos) = signal(Vec::<Item>::new());
    let (status_msg, set_status_msg) = signal(None);

    let (is_loading, set_is_loading) = signal(false);

    let update_query = move |ev| {
        let v = event_target_value(&ev);
        set_search_query.set(v);
    };

    let search_videos = move |ev: SubmitEvent| {
        ev.prevent_default();

        spawn_local(async move {
            let query = search_query.get_untracked();
            if query.is_empty() {
                set_status_msg.set(Some("Please enter a search query.".to_string()));
                return;
            }

            set_is_loading.set(true);
            set_status_msg.set(None);

            match services::search(query).await {
                Response::Success(videos) => set_videos.set(videos.items),
                Response::Failed(err) => set_status_msg.set(Some(format!("Search failed: {err}"))),
            };
            set_is_loading.set(false);
        });
    };

    view! {
        <main>
            <div class="navbar bg-base-100 shadow-sm text-neutral">
              <div class="flex-1">
                <a class="btn btn-ghost text-neutral text-xl">DoYou</a>
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
                  <button class="btn btn-neutral text-white" type="submit">"Search"</button>
                </form>

                <Show when=move || status_msg.get().is_some()>
                    <div role="alert" class="alert alert-error my-5">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                      </svg>
                      <span>{ move || status_msg.get().unwrap() }</span>
                    </div>
                </Show>

                <Show when=move || is_loading.get() fallback=move || view! {
					<ul class="list bg-base-100 rounded-box shadow-md">
						<For
							each=move || videos.get()
							key=|item| item.id.video_id.clone()
							children=move |item: Item| {view! {<MusicCard item=item/>} }
						/>
				   </ul>
				}>
                    <div class="flex h-screen justify-center items-center">
                        <span class="loading loading-spinner text-neutral size-30"></span>
                    </div>
                </Show>
            </div>
            
            <div class="fixed bottom-0 left-0 w-full bg-base-200 text-neutral shadow-inner"> 
				<Player/>
			</div>
			
        </main>
    }
}
