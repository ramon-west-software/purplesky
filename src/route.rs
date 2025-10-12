use dioxus::prelude::*;
use crate::components::{Home, Navbar, Profile, Reels, Search, Upload};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/search")]
    Search {},
    #[route("/upload")]
    Upload {},
    #[route("/reels")]
    Reels {},
    #[route("/profile")]
    Profile {},
}