use dioxus::prelude::*;

#[derive(Clone)]
pub struct AlertContext {
    pub message: Signal<Option<String>>,
}

impl AlertContext {
    pub fn new() -> Self {
        Self {
            message: Signal::new(None),
        }
    }
}

pub fn use_alert() -> AlertContext {
    use_context::<AlertContext>()
}

#[component]
pub fn AlertProvider(children: Element) -> Element {
    let _alert = use_context_provider(AlertContext::new);
    rsx! { {children} }
}
