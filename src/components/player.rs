use leptos::prelude::*;

use crate::music_player::MusicPlayer;


#[component]
pub fn Player() -> impl IntoView {
	let music_player = use_context::<MusicPlayer>().unwrap();
	let playing = Memo::new(move |_| music_player.playing.get());
	
	view! {
		<Show when=move || playing.get().is_some()>
			<div class="container mx-auto p-4 flex items-center justify-between">
				<div class="flex items-center gap-4 w-1/3">
					<img src={move || playing.get().map(|i| i.snippet.thumbnails.medium.url).unwrap_or("https://via.placeholder.com/64".to_string()) } alt="Thumbnail" class="w-16 h-16 rounded-md object-cover" />
					<div>
						<p class="font-bold text-lg">{move || playing.get().map(|i| i.snippet.title).unwrap_or("Unknow title".to_string()) }</p>
						<p class="text-sm">{move || playing.get().map(|i| i.snippet.channel_title).unwrap_or("Unknow channel".to_string())}</p>
					</div>
				</div>
				<div class="flex items-center gap-2 justify-center flex-grow">
					<button class="btn btn-ghost btn-circle">
						<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24">
							<path d="M18 18V6l-8 6 8 6zM6 6h2v12H6V6z" />
						</svg>
					</button>
					<button class="btn btn-ghost btn-circle" on:click=move |_| music_player.toggle_play()>
						<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24">
							{move || if music_player.is_playing.get() {
								view! { <path d="M6 4h4v16H6zM14 4h4v16h-4z" /> }
							} else {
								view! { <path d="M5 3l14 9-14 9V3z" /> }
							}}
						</svg>
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
