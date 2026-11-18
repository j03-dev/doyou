use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdHardDrive};

#[component]
pub fn DownloadedBadge(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        span { class: format!("badge badge-success badge-xs gap-1 {class}"),
            Icon { icon: LdHardDrive, class: "size-3" }
            "Downloaded"
        }
    }
}
