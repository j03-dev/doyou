use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdMusic};

/// An image that falls back to a music note when the artwork is missing or
/// fails to load.
///
/// Empty sources are common for local tracks with no embedded cover; failing
/// sources are common on Android, where the WebView refuses `file://` images.
/// Keeping the handling here means every track surface degrades the same way.
#[component]
pub fn Artwork(
    src: String,
    #[props(default)] alt: String,
    #[props(default)] class: &'static str,
    #[props(default = "size-10")] icon_class: &'static str,
) -> Element {
    let mut failed_src = use_signal(String::new);

    let has_src = !src.trim().is_empty();
    // Store which source failed, not just a boolean, so a track with a new
    // cover starts with a clean image even if this component is reused.
    let failed = has_src && failed_src.read().as_str() == src.as_str();
    let source_on_error = src.clone();

    rsx! {
        if !has_src || failed {
            div { class: format!("flex items-center justify-center bg-base-300 text-base-content/40 {class}"),
                Icon { icon: LdMusic, class: icon_class }
            }
        } else {
            img {
                class,
                src: "{src}",
                alt: "{alt}",
                loading: "lazy",
                onerror: move |_| failed_src.set(source_on_error.clone()),
            }
        }
    }
}
