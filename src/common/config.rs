use serde::{Deserialize, Serialize};
use std::fs;

type Error = Box<dyn std::error::Error>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub youtube_token: String,
}

pub fn load_config() -> Result<Option<AppConfig>, Error> {
    let path = get_config_path()?;
    if let Ok(content) = fs::read_to_string(&path) {
        let config = serde_json::from_str(&content)?;
        return Ok(Some(config));
    }
    Ok(None)
}

pub fn save_config(config: &AppConfig) -> Result<(), Error> {
    let path = get_config_path()?;

    if let Some(parent) = std::path::Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string(config)?;
    fs::write(&path, json)?;
    Ok(())
}

fn get_config_path() -> Result<String, Error> {
    let base_dir = get_android_files_dir()?;
    Ok(format!("{base_dir}/config.json"))
}

/// Gets the Android application's files directory path.
/// This is typically `/data/data/<package_name>/files/`
fn get_android_files_dir() -> Result<String, Error> {
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

    Ok(path_as_java_str.to_str()?.to_string())
}
