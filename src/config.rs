use std::fs;

use serde::{Deserialize, Serialize};

type Error = Box<dyn std::error::Error>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub yt_token: String,
}

pub fn load_config() -> Result<AppConfig, Error> {
    let path = get_config_path();
    let content = fs::read_to_string(&path)?;
    let config = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn save_config(config: &AppConfig) -> Result<(), Error> {
    let path = get_config_path();

    if let Some(parent) = std::path::Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string(config)?;
    fs::write(&path, json)?;
    Ok(())
}

fn get_config_path() -> String {
    #[cfg(target_os = "android")]
    {
        let base_dir = get_android_files_dir().unwrap_or_else(|_| "/data/local/tmp".to_string());
        format!("{base_dir}/config.json")
    }

    #[cfg(not(target_os = "android"))]
    {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        path.push("doyou");
        path.push("config.json");
        path.to_string_lossy().to_string()
    }
}

fn get_android_files_dir() -> Result<String, Box<dyn std::error::Error>> {
    let android_context = ndk_context::android_context();
    let java_vm = unsafe { jni::JavaVM::from_raw(android_context.vm().cast()) }?;
    let mut java = java_vm.attach_current_thread()?;
    let fs = unsafe { jni::objects::JObject::from_raw(android_context.context().cast()) };

    let files_dir = java
        .call_method(fs, "getFilesDir", "()Ljava/io/File;", &[])?
        .l()?;

    let files_dir: jni::objects::JString = java
        .call_method(&files_dir, "toString", "()Ljava/lang/String;", &[])?
        .l()?
        .try_into()?;

    let files_dir = java.get_string(&files_dir)?;
    Ok(files_dir.to_str()?.to_string())
}
