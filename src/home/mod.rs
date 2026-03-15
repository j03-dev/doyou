use dioxus::prelude::*;

use crate::common::components::alert_message::AlertMessage;
use crate::common::components::button::IconButton;
use crate::common::components::icons::{DoYouIcon, SearchIcon};
use crate::common::components::music_list::MusicList;
use crate::common::components::navbar::{NavBar, NavBarItem, NavBarPos};
use crate::common::components::text_input::TextInput;
use crate::common::context::{use_alert, use_settings};
use crate::core::utils::get_value_from;

use self::theme_controller::ThemeController;
use self::token_from::TokenForm;

mod theme_controller;
mod token_from;

#[derive(Clone, PartialEq)]
enum SearchMode {
    Home,
    Search(String),
}

#[component]
pub fn Home() -> Element {
    let mut alert = use_alert();
    let settings = use_settings();

    let mut show_search = use_signal(|| false);
    let mut search_mode = use_signal(|| SearchMode::Home);

    let items = use_resource(move || {
        let token = settings.general.read().youtube_token.clone();
        let mode = search_mode.read().clone();
        async move {
            match token {
                Some(t) => match mode {
                    SearchMode::Home => {
                        if let Err(err) = yt::data_api::home(&t).await.map(|r| r.items) {
                            alert.message.set(Some(err.to_string));
                        }
                        Ok(Vec::new())
                    }
                    SearchMode::Search(q) => yt::data_api::search(&q, &t).await.map(|r| r.items),
                },
                None => Ok(Vec::new()),
            }
        }
    });

    use_effect(move || {
        spawn(async move {
            if let Some(token) = settings.general.read().youtube_token.as_ref()
                && !token.trim_end().is_empty()
            {
                dbg!(token);
                document::eval("token_form.close()");
            } else {
                dbg!("displayb dialog");
                document::eval("token_form.showDialog()");
            }
        });
    });

    let search = move |evt: Event<FormData>| {
        let search_query = get_value_from(evt, "search");
        if let Some(q) = search_query {
            search_mode.set(SearchMode::Search(q));
            show_search.set(false);
        } else {
            alert
                .message
                .set(Some("Please enter a search query.".to_string()));
        }
    };

    let toggle_search = move |_| {
        show_search.set(!show_search());
        if !show_search() {
            search_mode.set(SearchMode::Home);
        }
    };

    rsx! {
        NavBar {
            NavBarItem { position: NavBarPos::Start, ThemeController {} }
            NavBarItem { position: NavBarPos::Center,
                if show_search() {
                    form { onsubmit: search,
                        TextInput {
                            name: "search",
                            r#type: "search",
                            placeholder: "Search",
                            SearchIcon { class: "h-[1em] opacity-50" }
                        }
                    }
                } else {
                    DoYouIcon {}
                }
            }
            NavBarItem { position: NavBarPos::End,
                IconButton { on_click: toggle_search, SearchIcon {} }
            }
        }

        TokenForm {}

        div { class: "m-2 pb-5",
            if let Some(message) = &*alert.message.read() {
                AlertMessage { message: message.clone() }
            }
            if let Some(Ok(items)) = &*items.read_unchecked() {
                MusicList { items: items.clone() }
            }
        }

    }
}
