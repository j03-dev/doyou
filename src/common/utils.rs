use dioxus::prelude::{Event, FormData, FormValue};

pub fn get_value_from(event: Event<FormData>, key: &'static str, ) -> String {
    event
        .get_first(key)
        .and_then(|v| match v {
            FormValue::Text(value) => Some(value),
            _ => None,
        })
        .unwrap_or_default()
}
