use dioxus::prelude::*;

use crate::core::playback::Playback;

#[component]
pub fn PlaybackProvider(children: Element) -> Element {
    let _playback = use_context_provider(|| Playback::new("audio"));
    rsx! {
        {children}
    }
}

pub fn use_playback() -> Playback {
    use_context::<Playback>()
}
