use dioxus::prelude::*;

use yt::data_api::types::Item;

use crate::context::AlertProps;
use crate::core::platform;
use crate::repository;

#[component]
pub fn PlaybackProvider(children: Element) -> Element {
    use_context_provider(|| PlaybackContext::new("audio"));
    let playback = use_playback();

    use_effect(move || {
        let mut playback = playback;
        let mut events = platform::player_events();
        spawn(async move {
            while let Some(event) = events.recv().await {
                match event.as_str() {
                    "ended" | "next" => playback.playback_controller(1),
                    "prev" => playback.playback_controller(-1),
                    "state:1" => playback.is_playing.set(true),
                    "state:0" => playback.is_playing.set(false),
                    _ => {
                        if let Some(rest) = event.strip_prefix("progress:") {
                            let mut parts = rest.splitn(2, ':');
                            if let Some(time) = parts.next().and_then(|v| v.parse::<f64>().ok()) {
                                playback.current_time.set(time);
                            }
                            if let Some(len) = parts.next().and_then(|v| v.parse::<f64>().ok())
                                && len.is_finite()
                                && len > 0.0
                            {
                                playback.duration.set(len);
                            }
                        } else if let Some(code) = event.strip_prefix("error:") {
                            playback.error.set(Some(AlertProps::error(format!(
                                "Playback failed (code: {})",
                                code
                            ))));
                        }
                    }
                }
            }
        });
    });

    rsx! {
        {children}
    }
}

pub fn use_playback() -> PlaybackContext {
    use_context::<PlaybackContext>()
}

#[derive(Clone, Copy, PartialEq)]
pub struct PlaybackContext {
    pub id: &'static str,
    pub is_playing: Signal<bool>,
    pub playing: Signal<Option<Item>>,
    pub queue: Signal<Vec<Item>>,
    pub current_index: Signal<usize>,
    pub is_loading: Signal<bool>,
    pub current_time: Signal<f64>,
    pub duration: Signal<f64>,
    pub error: Signal<Option<AlertProps>>,
}

impl PlaybackContext {
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            is_playing: Signal::new(false),
            playing: Signal::new(None),
            queue: Signal::new(Vec::new()),
            current_index: Signal::new(0),
            is_loading: Signal::new(false),
            current_time: Signal::new(0.0),
            duration: Signal::new(0.0),
            error: Signal::new(None),
        }
    }

    pub fn set_queue(&self, items: Vec<Item>) {
        let mut queue = self.queue;
        queue.set(items);
    }

    pub fn start(&self, index: usize) {
        let item = match self.queue.read().get(index).cloned() {
            Some(item) => item,
            _ => return,
        };

        let id = self.id;
        let mut is_playing = self.is_playing;
        let mut playing = self.playing;
        let mut current_index = self.current_index;
        let mut is_loading = self.is_loading;
        let mut current_time = self.current_time;
        let mut duration = self.duration;
        let mut error = self.error;

        spawn(async move {
            current_index.set(index);
            error.set(None);
            playing.set(Some(item.clone()));
            is_playing.set(true);
            is_loading.set(true);
            current_time.set(0.0);
            duration.set(0.0);

            match repository::youtube::audio_url(&item.id.as_string().unwrap()).await {
                Ok(src) => {
                    platform::media_play(
                        id,
                        &src,
                        &item.snippet.title,
                        &item.snippet.channel_title,
                    );
                }
                Err(e) => {
                    error.set(Some(AlertProps::error(format!(
                        "Failed to get audio: {}",
                        e
                    ))));
                    is_playing.set(false);
                    platform::media_stop(id);
                }
            };
            is_loading.set(false);
        });
    }

    pub fn play(&self) {
        let id = self.id;
        let mut is_playing = self.is_playing;
        platform::media_resume(id);
        is_playing.set(true);
    }

    pub fn pause(&self) {
        let id = self.id;
        let mut is_playing = self.is_playing;
        platform::media_pause(id);
        is_playing.set(false);
    }

    pub fn toggle_play(&self) {
        if *self.is_playing.read() {
            self.pause();
        } else {
            self.play();
        }
    }

    pub fn playback_controller(&self, delta: isize) {
        let len = self.queue.read().len();
        if len == 0 {
            return;
        }
        let current = *self.current_index.read();
        let new_index = (current as isize + delta).rem_euclid(len as isize) as usize;
        self.start(new_index);
    }

    pub fn update_current_time(&self) {
        let id = self.id;
        let mut current_time = self.current_time;
        spawn(async move {
            let mut eval = document::eval(&format!(
                r#"
                    const audio = document.getElementById('{}')
                    if (audio) dioxus.send(audio.currentTime)
                "#,
                id
            ));

            if let Ok(time) = eval.recv::<f64>().await {
                current_time.set(time);
            }
        });
    }

    pub fn update_duration(&self) {
        let id = self.id;
        let mut duration = self.duration;
        spawn(async move {
            let mut eval = document::eval(&format!(
                r#"
                    const audio = document.getElementById('{}')
                    if (audio) dioxus.send(audio.duration)
                "#,
                id
            ));

            if let Ok(len) = eval.recv::<f64>().await
                && len.is_finite()
                && len > 0.0
            {
                duration.set(len);
            }
        });
    }
}
