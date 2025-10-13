use dioxus::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct LoginRequest<'a> {
    identifier: &'a str,
    password: &'a str,
}

#[derive(Deserialize)]
struct LoginResponse {
    accessJwt: String,
    // you can also deserialize did, handle, refreshJwt, etc.
}

#[component]
pub fn Login(on_login: EventHandler<String>) -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);

    let submit = move |_| {
        let username = username.read().clone();
        let password = password.read().clone();
        let on_login = on_login.clone();
        loading.set(true);
        error.set(None);

        spawn(async move {
            let client = Client::new();
            let resp = client
                .post("https://bsky.social/xrpc/com.atproto.server.createSession")
                .json(&LoginRequest {
                    identifier: &username,
                    password: &password,
                })
                .send()
                .await;

            match resp {
                Ok(r) => {
                    if r.status().is_success() {
                        match r.json::<LoginResponse>().await {
                            Ok(body) => on_login.call(body.accessJwt),
                            Err(e) => error.set(Some(format!("Failed to parse response: {}", e))),
                        }
                    } else if r.status().as_u16() == 401 {
                        error.set(Some(
                            "Login failed. Username, email, or password are incorrect.".to_string(),
                        ));
                    } else {
                        error.set(Some(format!(
                            "Login failed. Server returned an error: {}",
                            r.status()
                        )));
                    }
                }
                Err(e) => error.set(Some(format!("Login failed. Unable to reach the atproto authenticator service. Error: {}", e))),
            }
            loading.set(false);
        });
    };

    rsx! {
        div { class: "login-page",
            h1 { "Login to Purplesky" }

            if let Some(err) = error.read().as_ref() {
                p { style: "color:red", "{err}" }
            }

            input {
                placeholder: "Username or email",
                value: "{username}",
                oninput: move |e| username.set(e.value()),
            }
            input {
                r#type: "password",
                placeholder: "Password",
                value: "{password}",
                oninput: move |e| password.set(e.value()),
            }

            button {
                onclick: submit,
                disabled: *loading.read(),
                if *loading.read() { "Logging in..." } else { "Login" }
            }
        }
    }
}
