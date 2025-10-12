use dioxus::prelude::*;

use crate::components::Hero;

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            Hero {}
            p { "This is the home page." }
        }
    }
}
