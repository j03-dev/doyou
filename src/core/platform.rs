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

/// Push the current track metadata and playback state to the Android
/// foreground playback service (notification + MediaSession).
///
/// No-op on platforms other than Android.
#[cfg(all(feature = "mobile", target_os = "android"))]
pub fn android_media_update(title: &str, artist: &str, is_playing: bool) {
    let android_context = ndk_context::android_context();
    let java_vm = unsafe { jni::JavaVM::from_raw(android_context.vm().cast()) };

    let result = java_vm.attach_current_thread(|env| -> Result<(), Error> {
        let activity =
            unsafe { jni::objects::JObject::from_raw(env, android_context.context().cast()) };
        let main_activity = env.get_object_class(&activity)?;
        let title = env.new_string(title)?;
        let artist = env.new_string(artist)?;

        env.call_static_method(
            &main_activity,
            jni::jni_str!("startOrUpdatePlayback"),
            jni::jni_sig!("(Ljava/lang/String;Ljava/lang/String;Z)V"),
            &[
                (&title).into(),
                (&artist).into(),
                jni::objects::JValue::Bool(is_playing.into()),
            ],
        )?;

        Ok(())
    });

    if let Err(err) = result {
        eprintln!("android_media_update failed: {err}");
    }
}

#[cfg(not(all(feature = "mobile", target_os = "android")))]
pub fn android_media_update(_title: &str, _artist: &str, _is_playing: bool) {}
