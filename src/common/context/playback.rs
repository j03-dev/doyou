use dioxus::prelude::*;

use yt::data_api::types::Item;
use yt::extractor::YouTubeExtractor;

use crate::common::components::music_player::MusicPlayer;

#[component]
pub fn PlaybackProvider(children: Element) -> Element {
    let playback = use_context_provider(|| PlaybackContext::new("audio"));

    rsx! {
        {children}
        div { class: "hidden",
            audio {
                id: playback.id,
                onended: move |_| playback.playback_controller(1),
                ontimeupdate: move |_| playback.update_current_time(),
                ondurationchange: move |_| playback.update_duration(),
            }
        }
        if playback.playing.read().is_some() {
            MusicPlayer {}
        }
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
    pub error: Signal<Option<String>>,
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
        let mut error = self.error;

        spawn(async move {
            current_index.set(index);
            error.set(None);
            playing.set(Some(item.clone()));
            is_loading.set(true);

            match YouTubeExtractor::default()
                .get_best_audio_url(&item.id.as_string().unwrap())
                .await
            {
                Ok(src) => {
                    let _ = document::eval(&format!(
                        r#"
                           let audio = document.getElementById('{}')
                           if (audio) {{
                               audio.src = '{}'
                               audio.play()
                           }}
                        "#,
                        id, src
                    ));
                }
                Err(e) => {
                    error.set(Some(format!("Failed to get audio: {}", e)));
                    is_playing.set(false);
                    let _ = document::eval(&format!(
                        r#"
                           let audio = document.getElementById('{}')
                           if (audio) audio.pause()
                        "#,
                        id
                    ));
                }
            };
            is_loading.set(false);
        });
    }

    pub fn play(&self) {
        let id = self.id;
        let mut is_playing = self.is_playing;
        let _ = document::eval(&format!(
            r#"
               let audio = document.getElementById('{}')
               if (audio) audio.play()
            "#,
            id
        ));
        is_playing.set(true);
    }

    pub fn pause(&self) {
        let id = self.id;
        let mut is_playing = self.is_playing;
        let _ = document::eval(&format!(
            r#"
               let audio = document.getElementById('{}')
               if (audio) audio.pause()
            "#,
            id
        ));
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
