use serde::{Deserialize, Serialize};

/// Represents one authenticated session.
#[derive(Clone, Debug)]
pub struct AuthState {
    pub token: String,
    pub handle: String,
}

/// Top-level authentication manager for the app.
/// Holds multiple accounts and tracks the active one.
#[derive(Clone, Debug)]
pub struct AuthSessionManager {
    pub accounts: Vec<AuthState>,
    pub active_handle: Option<String>,
}

impl AuthSessionManager {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
            active_handle: None,
        }
    }

    /// Add a new account and set it as active.
    pub fn add_account(&mut self, auth: AuthState) {
        // If the handle already exists, replace it
        if let Some(existing) = self.accounts.iter_mut().find(|a| a.handle == auth.handle) {
            *existing = auth.clone();
        } else {
            self.accounts.push(auth.clone());
        }

        self.active_handle = Some(auth.handle);
    }

    /// Get the currently active account.
    pub fn active_account(&self) -> Option<&AuthState> {
        self.active_handle
            .as_ref()
            .and_then(|h| self.accounts.iter().find(|a| &a.handle == h))
    }

    /// Log out of the active account (optional for future UI)
    pub fn remove_active_account(&mut self) {
        if let Some(active) = self.active_handle.clone() {
            self.accounts.retain(|a| a.handle != active);
            self.active_handle = self.accounts.first().map(|a| a.handle.clone());
        }
    }
}

#[derive(Serialize)]
pub struct LoginRequest<'a> {
    pub identifier: &'a str,
    pub password: &'a str,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "accessJwt")]
    pub access_jwt: String,
    pub handle: String,
}
