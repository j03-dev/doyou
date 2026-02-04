use dioxus::prelude::*;
use yt::data_api::types::Item;
use yt::extractor::YouTubeExtractor;

#[derive(Clone, Copy, PartialEq)]
pub struct Playback {
    pub id: &'static str,
    pub is_playing: Signal<bool>,
    pub playing: Signal<Option<Item>>,
    pub playlist: Signal<Vec<Item>>,
    pub current_index: Signal<usize>,
    pub is_loading: Signal<bool>,
    pub current_time: Signal<f64>,
    pub duration: Signal<f64>,
}

impl Playback {
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            is_playing: Signal::new(false),
            playing: Signal::new(None),
            playlist: Signal::new(Vec::new()),
            current_index: Signal::new(0),
            is_loading: Signal::new(false),
            current_time: Signal::new(0.0),
            duration: Signal::new(0.0),
        }
    }

    pub fn start(&mut self, index: usize) {
        let item = match self.playlist.read().get(index).cloned() {
            Some(item) => item,
            _ => return,
        };

        let mut state = *self;
        self.is_playing.set(true);

        let youtube_extractor = YouTubeExtractor::new();

        spawn(async move {
            state.current_index.set(index);
            state.is_loading.set(true);
            match youtube_extractor
                .get_best_audio_url(&item.id.as_string().unwrap())
                .await
            {
                Ok(src) => {
                    state.playing.set(Some(item));
                    let _ = document::eval(&format!(
                        r#"
                           let audio = document.getElementById('{id}')
                           if (audio) {{
                               audio.src = '{src}'
                               audio.play()
                           }}
                       "#,
                        id = state.id
                    ));
                }
                Err(e) => {
                    let _ = document::eval(&format!("console.log({e})"));
                    state.is_playing.set(false);
                }
            };
            state.is_loading.set(false);
        });
    }

    pub fn play(&mut self) {
        let _ = document::eval(&format!(
            r#"
               let audio = document.getElementById('{id}')
               if (audio) audio.play()
            "#,
            id = self.id
        ));
        self.is_playing.set(true);
    }

    pub fn pause(&mut self) {
        let _ = document::eval(&format!(
            r#"
               let audio = document.getElementById('{id}')
               if (audio) audio.pause()
            "#,
            id = self.id
        ));
        self.is_playing.set(false);
    }

    pub fn toggle_play(&mut self) {
        if *self.is_playing.read() {
            self.pause();
        } else {
            self.play();
        }
    }

    pub fn playback_controller(&mut self, delta: isize) {
        let len = self.playlist.read().len();
        if len == 0 {
            return;
        }
        let current = *self.current_index.read();
        let new_index = (current as isize + delta).rem_euclid(len as isize) as usize;
        self.start(new_index);
    }

    pub fn update_current_time(&mut self) {
        let id = self.id;
        let mut state = *self;
        spawn(async move {
            let mut eval = document::eval(&format!(
                r#"
                    const audio = document.getElementById('{id}')
                    if (audio) dioxus.send(audio.currentTime)
                "#
            ));

            if let Ok(current_time) = eval.recv::<f64>().await {
                state.current_time.set(current_time);
            }
        });
    }

    pub fn update_duration(&mut self) {
        let id = self.id;
        let mut state = *self;
        spawn(async move {
            let mut eval = document::eval(&format!(
                r#"
                    const audio = document.getElementById('{id}')
                    if (audio) dioxus.send(audio.duration)
                "#
            ));

            if let Ok(duration) = eval.recv::<f64>().await
                && duration.is_finite()
                && duration > 0.0
            {
                state.duration.set(duration);
            }
        });
    }
}
