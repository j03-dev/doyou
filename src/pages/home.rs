use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdMenu, LdSearch, LdX};

use crate::components::alert::Alert;
use crate::components::button::ButtonGhost;
use crate::components::form::get_value_from;
use crate::components::loading::LoadingSpinner;
use crate::components::music_card::MusicCard;
use crate::components::navbar::{NavBar, NavBarItem, NavBarPos};
use crate::components::text_input::TextInput;
use crate::context::{use_home, use_settings};

#[component]
pub fn Home() -> Element {
    let settings = use_settings();
    let home = use_home();

    let mut show_search = use_signal(|| false);

    let home_error = home.error;
    let home_is_loading = home.is_loading;
    let home_items = home.items;
    let settings_error = settings.error;

    use_effect(move || {
        if settings.general.read().youtube_token.is_none() {
            document::eval("token_form.showModal()");
        }
    });

    use_effect(move || {
        home.load_feed();
    });

    let search = move |evt: Event<FormData>| {
        evt.prevent_default();
        let search_query = get_value_from(evt, "search").unwrap_or_default();
        home.search(search_query);
    };

    let submit_token = move |evt: Event<FormData>| {
        evt.prevent_default();
        let token = get_value_from(evt, "token");
        settings.save_token(token.unwrap());
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
                            Icon { icon: LdSearch, fill: "grey", width: 16 }
                        }
                    }
                } else {
                    p { class: "btn btn-ghost text-xl", "DoYou" }
                }
            }
            NavBarItem { position: NavBarPos::End,
                ButtonGhost { onclick: move |_| show_search.set(!show_search()),
                    Icon { icon: LdSearch }
                }
            }
        }

        div { class: "m-2 pb-5",
            if let Some(alert_props) = home_error() {
                Alert { ..alert_props }
            }
            if let Some(alert_props) = settings_error() {
                Alert { ..alert_props }
            }
            if home_is_loading() {
                div { class: "flex h-screen justify-center items-center",
                    LoadingSpinner { size: 20 }
                }
            } else {
                div { class: "carousel carousel-center w-full gap-3",
                    for (index, item) in home_items().iter().enumerate() {
                        div { class: "carousel-item w-36 sm:w-40",
                            MusicCard { item: item.clone(), index }
                        }
                    }
                }
            }
        }

        dialog { id: "token_form", class: "modal",
            div { class: "modal-box w-96",
                form { method: "dialog",
                    button { class: "btn btn-sm absolute right-4 top-7",
                        Icon { icon: LdX }
                    }
                }
                br {}
                form { onsubmit: submit_token,
                    legend { class: "fieldset-legend", "Youtube Token" }
                    TextInput {
                        name: "token",
                        r#type: "password",
                        placeholder: "paste your api key here (e.g. AIzaSy...)",
                    }
                    button { class: "btn btn-primary mt-%", r#type: "submit", "Save" }
                }
            }
        }

    }
}

#[component]
fn ThemeController() -> Element {
    let settings = use_settings();
    let themes = &["Lofi", "Black", "Night", "Forest", "Dracula"];

    rsx! {
        div { class: "dropdown",
            div {
                tabindex: 0,
                role: "button",
                class: "btn btn-ghost btn-circle",
                Icon { icon: LdMenu }
            }
            ul {
                tabindex: -1,
                class: "dropdown-content bg-base-300 rounded-box z-1 w-52 p-2 shadow-2xl",
                for theme in themes {
                    ThemeItem {
                        name: theme,
                        callback: move |theme| {
                            settings.save_theme(theme);
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn ThemeItem(name: &'static str, callback: Callback<String>) -> Element {
    rsx! {
        li {
            input {
                r#type: "radio",
                name: "theme-dropdown",
                class: "theme-controller w-full btn btn-sm btn-block btn-ghost justify-start",
                aria_label: name,
                value: name.to_lowercase(),
                onclick: move |_| callback.call(name.to_lowercase()),
            }
        }
    }
}
