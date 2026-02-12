use dioxus::prelude::{Event, FormData, FormValue};

pub fn get_form_value(key: &'static str, event: Event<FormData>) -> String {
    event
        .get_first(key)
        .and_then(|v| match v {
            FormValue::Text(value) => Some(value),
            _ => None,
        })
        .unwrap_or_default()
}
