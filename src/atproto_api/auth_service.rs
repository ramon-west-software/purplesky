use serde::{Deserialize, Serialize};
use reqwest::Client;
use crate::auth::{AuthState, AuthSessionManager}; // keep session manager in src/auth.rs

/// Represents a service to interact with the Bluesky auth API.
pub struct AuthService {
    client: Client,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Login with username/email and password.
    /// Returns AuthState on success.
    pub async fn login(
        &self,
        identifier: &str,
        password: &str,
    ) -> Result<AuthState, String> {
        let req = LoginRequest { identifier, password };

        let resp = self
            .client
            .post("https://bsky.social/xrpc/com.atproto.server.createSession")
            .json(&req)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if resp.status().as_u16() == 401 {
            return Err("Login failed. Username, email, or password are incorrect.".into());
        }

        if !resp.status().is_success() {
            return Err(format!("Login failed. Server returned: {}", resp.status()));
        }

        let body: LoginResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(AuthState {
            token: body.access_jwt,
            handle: body.handle,
        })
    }
}

/// Request payload for login.
#[derive(Serialize)]
pub struct LoginRequest<'a> {
    pub identifier: &'a str,
    pub password: &'a str,
}

/// Response payload from the API.
#[derive(Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "accessJwt")]
    pub access_jwt: String,
    pub handle: String,
}
