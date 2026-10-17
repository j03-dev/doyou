use crate::core::error::Error;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

pub fn local_music_dirs() -> Vec<PathBuf> {
    [
        "/storage/emulated/0/Music",
        "/storage/emulated/0/Download",
        "/storage/emulated/0/Podcasts",
    ]
    .iter()
    .map(PathBuf::from)
    .collect()
}

const LOCAL_URI_SET: &percent_encoding::AsciiSet = &percent_encoding::NON_ALPHANUMERIC.remove(b'/');

pub fn local_media_url(path: &str) -> Result<String, Error> {
    let encoded = percent_encoding::utf8_percent_encode(path, LOCAL_URI_SET);
    Ok(format!("file://{encoded}"))
}

pub fn has_audio_permission() -> bool {
    let mut granted = false;
    call_main_activity(|env, class| {
        granted = env
            .call_static_method(
                class,
                jni::jni_str!("hasAudioPermission"),
                jni::jni_sig!("()Z"),
                &[],
            )?
            .z()?;
        Ok(())
    })
    .ok();
    granted
}

pub async fn request_audio_permission() -> bool {
    if has_audio_permission() {
        return true;
    }

    let (tx, rx) = tokio::sync::oneshot::channel();
    {
        let mut pending = PERMISSION_TX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        *pending = Some(tx);
    }

    call_main_activity(|env, class| {
        env.call_static_method(
            class,
            jni::jni_str!("requestAudioPermission"),
            jni::jni_sig!("()V"),
            &[],
        )?;
        Ok(())
    })
    .ok();

    rx.await.unwrap_or(false)
}

pub fn get_config_dir() -> Result<PathBuf, Error> {
    let android_context = ndk_context::android_context();
    let java_vm = java_vm();
    java_vm.attach_current_thread(|env| -> Result<PathBuf, Error> {
        let context =
            unsafe { jni::objects::JObject::from_raw(env, android_context.context().cast()) };
        let files_dir = env
            .call_method(
                &context,
                jni::jni_str!("getFilesDir"),
                jni::jni_sig!("()Ljava/io/File;"),
                &[],
            )?
            .l()?;
        let path = env
            .call_method(
                &files_dir,
                jni::jni_str!("toString"),
                jni::jni_sig!("()Ljava/lang/String;"),
                &[],
            )?
            .l()?;
        let path = env.cast_local::<jni::objects::JString>(path)?;
        let path = path.try_to_string(env)?;
        Ok(PathBuf::from(path))
    })
}

pub fn media_play(_id: &str, src: &str, title: &str, artist: &str) {
    call_main_activity(|env, class| {
        let src = env.new_string(src)?;
        let title = env.new_string(title)?;
        let artist = env.new_string(artist)?;
        env.call_static_method(
            class,
            jni::jni_str!("playTrack"),
            jni::jni_sig!("(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V"),
            &[(&src).into(), (&title).into(), (&artist).into()],
        )?;
        Ok(())
    })
    .ok();
}

pub fn media_pause(_id: &str) {
    call_main_activity(|env, class| {
        env.call_static_method(
            class,
            jni::jni_str!("pausePlayback"),
            jni::jni_sig!("()V"),
            &[],
        )?;

        Ok(())
    })
    .ok();
}

pub fn media_resume(_id: &str) {
    call_main_activity(|env, class| {
        env.call_static_method(
            class,
            jni::jni_str!("resumePlayback"),
            jni::jni_sig!("()V"),
            &[],
        )?;
        Ok(())
    })
    .ok();
}

pub fn media_stop(_id: &str) {
    call_main_activity(|env, class| {
        env.call_static_method(
            class,
            jni::jni_str!("stopPlayback"),
            jni::jni_sig!("()V"),
            &[],
        )?;

        Ok(())
    })
    .ok();
}

pub fn player_events() -> tokio::sync::mpsc::UnboundedReceiver<String> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let mut sender = PLAYER_EVENT_TX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *sender = Some(tx);
    rx
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_dioxus_main_MainActivity_onPlayerEvent<'caller>(
    mut unowned_env: jni::EnvUnowned<'caller>,
    _class: jni::objects::JClass<'caller>,
    event: jni::objects::JString<'caller>,
) {
    let result = unowned_env.with_env(|env| -> jni::errors::Result<()> {
        let event = event.try_to_string(env)?;

        match event.as_str() {
            "perm:granted" => resolve_permission(true),
            "perm:denied" => resolve_permission(false),
            _ => {}
        }

        let sender = PLAYER_EVENT_TX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if let Some(sender) = sender.as_ref() {
            let _ = sender.send(event);
        }
        Ok(())
    });

    result.resolve::<jni::errors::ThrowRuntimeExAndDefault>();
}

fn java_vm() -> &'static jni::JavaVM {
    static JAVA_VM: OnceLock<jni::JavaVM> = OnceLock::new();
    JAVA_VM.get_or_init(|| {
        let context = ndk_context::android_context();
        unsafe { jni::JavaVM::from_raw(context.vm().cast()) }
    })
}

static PLAYER_EVENT_TX: Mutex<Option<tokio::sync::mpsc::UnboundedSender<String>>> =
    Mutex::new(None);

static PERMISSION_TX: Mutex<Option<tokio::sync::oneshot::Sender<bool>>> = Mutex::new(None);

fn resolve_permission(granted: bool) {
    let sender = PERMISSION_TX
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .take();

    if let Some(sender) = sender {
        let _ = sender.send(granted);
    }
}

static MAIN_ACTIVITY: Mutex<Option<jni::objects::Global<jni::objects::JObject<'static>>>> =
    Mutex::new(None);

fn call_main_activity(
    f: impl for<'a> FnOnce(&mut jni::Env<'a>, &jni::objects::JClass<'a>) -> jni::errors::Result<()>,
) -> jni::errors::Result<()> {
    let mut activity = MAIN_ACTIVITY
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let java_vm = java_vm();
    java_vm.attach_current_thread(|env| {
        let class = match activity.as_ref() {
            Some(activity) => env.get_object_class(activity)?,
            None => {
                let context = ndk_context::android_context();
                let context =
                    unsafe { jni::objects::JObject::from_raw(env, context.context().cast()) };
                let global_context = env.new_global_ref(&context)?;
                let class = env.get_object_class(&global_context)?;
                *activity = Some(global_context);
                class
            }
        };
        f(env, &class)
    })
}
