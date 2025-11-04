use leptos::prelude::*;
use leptos::html::Audio;

use crate::types::Item;


#[derive(Clone, Copy)]
pub struct MusicPlayer {
    pub audio_ref: NodeRef<Audio>,
    pub is_playing: RwSignal<bool>,
    pub item: RwSignal<Option<Item>>,
}

impl MusicPlayer {
    pub fn new() -> Self {
        Self {
            audio_ref: NodeRef::new(),
            is_playing: RwSignal::new(false),
            item: RwSignal::new(None),
        }
    }

    pub fn toggle_play(&self, src: &str) {
        if let Some(audio) = self.audio_ref.get() {
            if audio.paused() {
                if audio.src().is_empty() {
                    audio.set_src(src);
                }
                assert!(audio.play().is_ok());
                self.is_playing.set(true);
            } else {
                assert!(audio.pause().is_ok());
                self.is_playing.set(false);
            }
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
			self.is_playing.set(true);
		}
	}
	
}
