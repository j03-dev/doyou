use dioxus::prelude::*;

#[component]
pub fn CoverCollage(
    #[props(default)] thumbnails: Vec<String>,
    #[props(default)] class: &'static str,
) -> Element {
    rsx! {
        div {
            class: format!("grid aspect-square grid-cols-2 grid-rows-2 overflow-hidden bg-base-300 {class}"),
            for index in 0..4 {
                if let Some(url) = thumbnails.get(index) {
                    img {
                        class: "size-full object-cover",
                        src: "{url}",
                        alt: "",
                        loading: "lazy",
                    }
                } else {
                    div { class: "bg-base-200" }
                }
            }
        }
    }
}
