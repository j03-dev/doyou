use dioxus::prelude::{Event, FormData, FormValue};

use crate::core::error::Error;

pub fn get_value_from(event: Event<FormData>, key: &'static str) -> String {
    event
        .get_first(key)
        .and_then(|v| match v {
            FormValue::Text(value) => Some(value),
            _ => None,
        })
        .unwrap_or_default()
}

pub fn get_config_path() -> Result<String, Error> {
    let base_dir = get_android_files_dir()?;
    Ok(format!("{base_dir}/config.json"))
}

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
