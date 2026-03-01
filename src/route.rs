use dioxus::prelude::*;

use crate::common::components::dock::Dock;
use crate::pages::{Favorite, Home, Setting};

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[layout(Dock)]
    #[route("/")]
    Home {},

    #[route("/favorite")]
    Favorite {},

    #[route("/setting")]
    Setting {},
}
