use std::path::PathBuf;

use dioxus::prelude::{Event, FormData, FormValue};

use crate::core::error::Error;

pub fn get_value_from(event: Event<FormData>, key: &'static str) -> Option<String> {
    event.get_first(key).and_then(|v| match v {
        FormValue::Text(value) => Some(value),
        _ => None,
    })
}

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
    let java_vm = unsafe { jni::JavaVM::from_raw(android_context.vm().cast()) }?;
    let mut jni_env = java_vm.attach_current_thread()?;

    let android_context_object =
        unsafe { jni::objects::JObject::from_raw(android_context.context().cast()) };

    let java_file_object = jni_env
        .call_method(
            android_context_object,
            "getFilesDir",
            "()Ljava/io/File;",
            &[],
        )?
        .l()?;

    let path_as_jstring: jni::objects::JString = jni_env
        .call_method(&java_file_object, "toString", "()Ljava/lang/String;", &[])?
        .l()?
        .try_into()?;

    let path_as_java_str = jni_env.get_string(&path_as_jstring)?;

    Ok(PathBuf::from(path_as_java_str.to_str()?))
}
