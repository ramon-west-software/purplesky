use crate::auth::AuthSessionManager;
use dioxus::prelude::*;

#[component]
pub fn Profile() -> Element {
    let session_manager = use_context::<Signal<AuthSessionManager>>();
    let active_handle = session_manager
        .read()
        .active_account()
        .map(|acc| acc.handle.clone())
        .unwrap_or_else(|| "Unknown".to_string());

    rsx! {
        div { class: "profile-container", // full screen
            div { class: "profile-header",
                h2 { "{active_handle}" },
            }
            div { class: "profile-page", // the card below header
                div { class: "profile-content",
                    p { "This is the profile page." },
                }
            }
        }
    }
}
