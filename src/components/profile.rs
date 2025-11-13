use crate::atproto_api::user_service::{UserProfile, UserService};
use crate::auth::AuthSessionManager;
use dioxus::prelude::*;

#[component]
pub fn Profile() -> Element {
    let session_manager = use_context::<Signal<AuthSessionManager>>();
    let use_effect_account = session_manager.read().active_account().cloned();
    let rsx_account = session_manager.read().active_account().cloned();

    // make the signals `mut` because we call `.set()` on them
    let profile_data = use_signal(|| Option::<UserProfile>::None);
    let loading = use_signal(|| false);
    let error = use_signal(|| Option::<String>::None);

    // Fetch user profile when the component mounts
    use_effect(move || {
        if let Some(acc) = use_effect_account.clone() {
            let mut profile_data = profile_data.clone();
            let mut loading = loading.clone();
            let mut error = error.clone();

            spawn(async move {
                loading.set(true);
                error.set(None);

                match UserService::get_profile(&acc.token, &acc.handle).await {
                    Ok(profile) => profile_data.set(Some(profile)),
                    Err(err) => error.set(Some(err)),
                }

                loading.set(false);
            });
        };

        // 👇 this closure is returned as the cleanup
        || {};
    });

    rsx! {
        div { class: "profile-container",

            div { class: "profile-header",
                if let Some(acc) = rsx_account.clone() {
                    h2 { "@{acc.handle}" }
                } else {
                    h2 { "Unknown" }
                }
            }

            div {
                if *loading.read() {
                    p { "Loading profile..." }
                } else if let Some(err) = error.read().as_ref() {
                    p { style: "color:red", "{err}" }
                } else if let Some(profile) = profile_data.read().as_ref() {
                    div { class: "profile-card",
                        if let (Some(avatar), Some(name), Some(pronouns), Some(desc)) = (&profile.avatar, &profile.display_name, &profile.pronouns, &profile.description) {
                            div {  class: "profile-card-info",
                                    img {
                                        src: "{avatar}",
                                        alt: "avatar",
                                        class: "profile-avatar"
                                    }
                                    div {
                                        p { "{name} ({pronouns})" }
                                        div { class: "profile-stats",
                                            span { "{profile.posts_count.unwrap_or(0)} posts" }
                                            span { "{profile.followers_count.unwrap_or(0)} followers" }
                                            span { "{profile.follows_count.unwrap_or(0)} following" }
                                        }
                                    }
                            }
                            p { "{desc}" }
                        }
                    }

                } else {
                    p { "No profile data." }
                }
            }

            div {
                "img"
             }
        }
    }
}
