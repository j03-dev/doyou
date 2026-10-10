use dioxus::prelude::*;

use crate::components::dock::Dock;
use crate::pages::{Favorite, Home, Playlist, Setting};

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[layout(Dock)]
    #[route("/")]
    Home {},

    #[route("/favorite")]
    Favorite {},

    #[route("/playlist/:id")]
    Playlist { id: i32 },

    #[route("/setting")]
    Setting {},
}
