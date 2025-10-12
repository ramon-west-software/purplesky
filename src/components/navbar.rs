use dioxus::prelude::*;
use crate::route::Route;

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Search {},
                "Search"
            }
            Link {
                to: Route::Upload {},
                "Upload"
            }
            Link {
                to: Route::Reels {},
                "Reels"
            }
            Link {
                to: Route::Profile {},
                "Profile"
            }
        }

        Outlet::<Route> {}
    }
}