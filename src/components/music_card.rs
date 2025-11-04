use leptos::prelude::*;
use leptos::{html::Audio, task::spawn_local};

use crate::services::{self, BASE_URL};
use crate::types::{Item, Response};

#[derive(Clone, Copy)]
struct Player {
	audio_ref: NodeRef<Audio>,
	is_playing: RwSignal<bool>,
}

impl Player {
	fn new() -> Self {
		Self { 
			audio_ref: NodeRef::new(),
			is_playing: RwSignal::new(false), 
		}
	}
	
    fn play(&self, src: &str) {
		if let Some(audio) = self.audio_ref.get() {
			if audio.paused() {
				if audio.src().is_empty() {audio.set_src(src);}
				assert!(audio.play().is_ok());
				self.is_playing.set(true);
			} else {
				assert!(audio.pause().is_ok());
				self.is_playing.set(false);
			}
		}
	}
}

#[component]
pub fn MusicCard(item: Item) -> impl IntoView {
    let (favorite_state, set_favorite_state) = signal(false);
    let (is_downloading, set_is_downloading) = signal(false);
    
    let player = Player::new();
    
    let is_playing = Memo::new(move |_| player.is_playing.get());

    let play_pause = move |_| {
		let video_id = item.id.video_id.clone();
		let player = *&player;
        spawn_local(async move {
			set_is_downloading.set(true);
			match services::download(video_id).await {
				Response::Success(downloaded) => {
					set_is_downloading.set(false);
					player.play(&format!("{BASE_URL}/listen?id={}", downloaded.video_id));
				}
				Response::Failed(_err) => set_is_downloading.set(false),
			};
		});
    };

    view! {
        <li class="list-row">
            <div>
                <img class="size-50 rounded-box" src={ item.snippet.thumbnails.medium.url.clone() } />
            </div>
            <div>
                <div> { item.snippet.title.clone() } </div>
                <div class="text-xs uppercase font-semibold opacite-60">
                    { item.snippet.channel_title.clone() }
                </div>
                <p class="list-col-wrap text-xs"> { item.snippet.description.clone() } </p>

                <audio node_ref=player.audio_ref on:ended=move |_| player.is_playing.set(false)/>

                <button class="btn btn-square btn-ghost" on:click=play_pause>
                    <Show when=move || is_downloading.get() fallback=move || view! {
                            <svg class="size-[1.2em]" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                                <g stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor">
                                <Show when=move || is_playing.get() fallback=move || view!{
									<path d="M6 3L20 12 6 21 6 3z"></path>
								}>
                                    <path d="M6 4H8V20H6zM16 4H18V20H16z"></path>
                                </Show>
                                </g>
                            </svg>
                          }>
                          <span class="loading loading-spinner"></span>
                    </Show>
                </button>


                <button class="btn btn-square btn-ghost" on:click=move |_| set_favorite_state.set(!favorite_state.get())>
                  <svg class="size-[1.2em]" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                        <g stroke-linejoin="round"
                           stroke-linecap="round"
                           stroke-width="2"
                           fill="none"
                           stroke="currentColor"
                           class={ move || {
                                    if favorite_state.get() {"fill-red-500 stroke-red-500".to_string()}
                                    else {"fill-transparent stroke-current".to_string()}
                                 }}
                         >
                            <path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z">
                            </path>
                        </g>
                    </svg>
                </button>
            </div>
       </li>
    }
}
