use leptos::html::Audio;
use leptos::prelude::*;

use commons::Item;

#[derive(Clone, Copy)]
pub struct MusicPlayer {
    pub audio_ref: NodeRef<Audio>,
    pub is_playing: RwSignal<bool>,
    pub playing: RwSignal<Option<Item>>,
    pub playlist: RwSignal<Vec<Item>>,
}

impl MusicPlayer {
    pub fn new() -> Self {
        Self {
            audio_ref: NodeRef::new(),
            is_playing: RwSignal::new(false),
            playing: RwSignal::new(None),
            playlist: RwSignal::new(Vec::new()),
        }
    }

    pub fn start(&self, src: &str) {
        if let Some(audio) = self.audio_ref.get() {
            audio.set_src(src); // allways set the src
            assert!(audio.play().is_ok());
            self.is_playing.set(true);
        }
    }

    pub fn play(&self) {
        if let Some(audio) = self.audio_ref.get() {
            assert!(audio.play().is_ok());
            self.is_playing.set(true);
        }
    }

    pub fn pause(&self) {
        if let Some(audio) = self.audio_ref.get() {
            assert!(audio.pause().is_ok());
            self.is_playing.set(false);
        }
    }

    pub fn toggle_play(&self) {
        if self.is_playing.get() {
            self.pause()
        } else {
            self.play()
        }
    }
}
