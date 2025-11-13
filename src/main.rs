use dioxus::prelude::*;
mod atproto_api;
mod auth;
mod components;
mod route;
use route::Route;
use components::Login;
use auth::AuthSessionManager;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    let mut session_manager = use_signal(AuthSessionManager::new);

    use_context_provider(|| session_manager);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        if session_manager.read().active_account().is_none() {
             Login { on_login: move |auth_state| session_manager.write().add_account(auth_state) }
        } else {
            Router::<Route> {}
        }
    }
}

