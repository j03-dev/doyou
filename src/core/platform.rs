use std::path::PathBuf;

use crate::core::error::Error;

pub fn get_config_path() -> Result<PathBuf, Error> {
    let config_dir = get_config_dir()?;
    let path = config_dir.join("config.db");
    if let Some(parent) = std::path::Path::new(&path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(path)
}

#[cfg(not(feature = "mobile"))]
pub fn get_config_dir() -> Result<PathBuf, Error> {
    let config_dir = directories::ProjectDirs::from("com", "doyou", "doyou")
        .ok_or("Failed to get config directory")?
        .config_dir()
        .to_path_buf();
    Ok(config_dir)
}

#[cfg(feature = "mobile")]
#[allow(dead_code)]
pub fn get_config_dir() -> Result<PathBuf, Error> {
    let base_dir = get_android_files_dir()?;
    Ok(base_dir)
}

#[cfg(feature = "mobile")]
fn get_android_files_dir() -> Result<PathBuf, Error> {
    let android_context = ndk_context::android_context();
    let java_vm = unsafe { jni::JavaVM::from_raw(android_context.vm().cast()) };

    java_vm.attach_current_thread(|env| -> Result<PathBuf, Error> {
        let context_object =
            unsafe { jni::objects::JObject::from_raw(env, android_context.context().cast()) };

        let java_file_object = env
            .call_method(
                &context_object,
                jni::jni_str!("getFilesDir"),
                jni::jni_sig!("()Ljava/io/File;"),
                &[],
            )?
            .l()?;

        let path_object = env
            .call_method(
                &java_file_object,
                jni::jni_str!("toString"),
                jni::jni_sig!("()Ljava/lang/String;"),
                &[],
            )?
            .l()?;

        let path_as_jstring = env.cast_local::<jni::objects::JString>(path_object)?;
        let path = path_as_jstring.try_to_string(env)?;

        Ok(PathBuf::from(path))
    })
}

#[cfg(target_os = "android")]
static JAVA_VM: std::sync::OnceLock<jni::JavaVM> = std::sync::OnceLock::new();

#[cfg(target_os = "android")]
static MAIN_ACTIVITY: std::sync::Mutex<
    Option<jni::objects::Global<jni::objects::JObject<'static>>>,
> = std::sync::Mutex::new(None);

#[cfg(target_os = "android")]
static PLAYER_EVENT_TX: std::sync::Mutex<Option<tokio::sync::mpsc::UnboundedSender<String>>> =
    std::sync::Mutex::new(None);

#[cfg(target_os = "android")]
fn call_main_activity(
    f: impl for<'a> FnOnce(&mut jni::Env<'a>, &jni::objects::JClass<'a>) -> jni::errors::Result<()>,
) -> jni::errors::Result<()> {
    let mut activity_guard = MAIN_ACTIVITY.lock().unwrap_or_else(|err| err.into_inner());
    let pending_context = if activity_guard.is_some() {
        None
    } else {
        Some(ndk_context::android_context())
    };
    let java_vm = JAVA_VM.get_or_init(|| {
        let context = ndk_context::android_context();
        unsafe { jni::JavaVM::from_raw(context.vm().cast()) }
    });

    java_vm.attach_current_thread(|env| {
        let class = match activity_guard.as_ref() {
            Some(activity) => env.get_object_class(activity)?,
            None => {
                let context = pending_context.expect("android context missing on first call");
                let activity =
                    unsafe { jni::objects::JObject::from_raw(env, context.context().cast()) };
                let activity = env.new_global_ref(&activity)?;
                let class = env.get_object_class(&activity)?;
                *activity_guard = Some(activity);
                class
            }
        };
        f(env, &class)
    })
}

#[cfg(target_os = "android")]
pub fn media_play(_id: &str, src: &str, title: &str, artist: &str) {
    let result = call_main_activity(|env, class| {
        let url = env.new_string(src)?;
        let title = env.new_string(title)?;
        let artist = env.new_string(artist)?;
        env.call_static_method(
            class,
            jni::jni_str!("playTrack"),
            jni::jni_sig!("(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V"),
            &[(&url).into(), (&title).into(), (&artist).into()],
        )?;
        Ok(())
    });
    if let Err(err) = result {
        eprintln!("media_play failed: {err}");
    }
}

#[cfg(target_os = "android")]
pub fn media_pause(_id: &str) {
    let result = call_main_activity(|env, class| {
        env.call_static_method(
            class,
            jni::jni_str!("pausePlayback"),
            jni::jni_sig!("()V"),
            &[],
        )?;
        Ok(())
    });
    if let Err(err) = result {
        eprintln!("media_pause failed: {err}");
    }
}

#[cfg(target_os = "android")]
pub fn media_resume(_id: &str) {
    let result = call_main_activity(|env, class| {
        env.call_static_method(
            class,
            jni::jni_str!("resumePlayback"),
            jni::jni_sig!("()V"),
            &[],
        )?;
        Ok(())
    });
    if let Err(err) = result {
        eprintln!("media_resume failed: {err}");
    }
}

#[cfg(target_os = "android")]
pub fn media_stop(_id: &str) {
    let result = call_main_activity(|env, class| {
        env.call_static_method(
            class,
            jni::jni_str!("stopPlayback"),
            jni::jni_sig!("()V"),
            &[],
        )?;
        Ok(())
    });
    if let Err(err) = result {
        eprintln!("media_stop failed: {err}");
    }
}

#[cfg(target_os = "android")]
pub fn player_events() -> tokio::sync::mpsc::UnboundedReceiver<String> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    *PLAYER_EVENT_TX
        .lock()
        .unwrap_or_else(|err| err.into_inner()) = Some(tx);
    rx
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_dioxus_main_MainActivity_onPlayerEvent<'caller>(
    mut unowned_env: jni::EnvUnowned<'caller>,
    _class: jni::objects::JClass<'caller>,
    event: jni::objects::JString<'caller>,
) {
    unowned_env
        .with_env(|env| -> jni::errors::Result<()> {
            let event = event.try_to_string(env)?;
            let sender = PLAYER_EVENT_TX
                .lock()
                .unwrap_or_else(|err| err.into_inner());
            if let Some(tx) = sender.as_ref() {
                let _ = tx.send(event);
            }
            Ok(())
        })
        .resolve::<jni::errors::ThrowRuntimeExAndDefault>();
}

#[cfg(not(target_os = "android"))]
pub fn media_play(id: &str, src: &str, _title: &str, _artist: &str) {
    let _ = dioxus::prelude::document::eval(&format!(
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

#[cfg(not(target_os = "android"))]
pub fn media_pause(id: &str) {
    let _ = dioxus::prelude::document::eval(&format!(
        r#"
            let audio = document.getElementById('{}')
            if (audio) audio.pause()
        "#,
        id
    ));
}

#[cfg(not(target_os = "android"))]
pub fn media_resume(id: &str) {
    let _ = dioxus::prelude::document::eval(&format!(
        r#"
            let audio = document.getElementById('{}')
            if (audio) audio.play()
        "#,
        id
    ));
}

#[cfg(not(target_os = "android"))]
pub fn media_stop(id: &str) {
    let _ = dioxus::prelude::document::eval(&format!(
        r#"
            let audio = document.getElementById('{}')
            if (audio) audio.pause()
        "#,
        id
    ));
}

#[cfg(not(target_os = "android"))]
pub fn player_events() -> tokio::sync::mpsc::UnboundedReceiver<String> {
    let (_tx, rx) = tokio::sync::mpsc::unbounded_channel();
    rx
}
