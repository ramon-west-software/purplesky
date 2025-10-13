use dioxus::prelude::*;
mod components;
mod route;
use route::Route;
use components::Login;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    let mut authToken = use_signal(|| Option::<String>::None);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        if authToken.read().is_none() {
             Login { on_login: move |t| authToken.set(Some(t)) }
        } else {
            Router::<Route> {}
        }
    }
}

