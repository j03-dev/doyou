use crate::core::error::Error;
use std::path::PathBuf;

pub fn get_config_dir() -> Result<PathBuf, Error> {
    directories::ProjectDirs::from("com", "doyou", "doyou")
        .map(|dirs| dirs.config_dir().to_path_buf())
        .ok_or_else(|| "Failed to get config directory".into())
}

pub fn media_play(id: &str, src: &str, _title: &str, _artist: &str) {
    let script = format!(
        r#"
        const audio = document.getElementById({id:?});
        if (audio) {{
            audio.src = {src:?};
            void audio.play();
        }}
        "#,
        id = id,
        src = src,
    );

    let _ = dioxus::prelude::document::eval(&script);
}

pub fn media_pause(id: &str) {
    let script = format!(
        r#"
        const audio = document.getElementById({id:?});
        if (audio) {{
            audio.pause();
        }}
        "#,
        id = id,
    );

    let _ = dioxus::prelude::document::eval(&script);
}

pub fn media_resume(id: &str) {
    let script = format!(
        r#"
        const audio = document.getElementById({id:?});
        if (audio) {{
            void audio.play();
        }}
        "#,
        id = id,
    );

    let _ = dioxus::prelude::document::eval(&script);
}

pub fn media_stop(id: &str) {
    let script = format!(
        r#"
        const audio = document.getElementById({id:?});
        if (audio) {{
            audio.pause();
            audio.currentTime = 0;
        }}
        "#,
        id = id,
    );

    let _ = dioxus::prelude::document::eval(&script);
}

pub fn player_events() -> tokio::sync::mpsc::UnboundedReceiver<String> {
    let (_tx, rx) = tokio::sync::mpsc::unbounded_channel();
    rx
}
