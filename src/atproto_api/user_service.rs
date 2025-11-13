use reqwest::Client;
use serde::Deserialize;

/// Represents the shape of the Bluesky user profile response
/// This uses `app.bsky.actor.getProfile`
#[derive(Debug, Deserialize, Clone)]
pub struct UserProfile {
    pub did: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub pronouns: Option<String>,
    #[serde(rename = "followersCount")]
    pub followers_count: Option<u64>,
    #[serde(rename = "followsCount")]
    pub follows_count: Option<u64>,
    #[serde(rename = "postsCount")]
    pub posts_count: Option<u64>,
    pub avatar: Option<String>,
}

pub struct UserService;

impl UserService {
    /// Fetch the profile info for the currently authenticated user
    pub async fn get_profile(token: &str, handle: &str) -> Result<UserProfile, String> {
        let url = format!(
            "https://bsky.social/xrpc/app.bsky.actor.getProfile?actor={}",
            handle
        );

        let client = Client::new();
        let resp = client
            .get(url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if resp.status().is_success() {
            resp.json::<UserProfile>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err(format!("Server returned {}", resp.status()))
        }
    }
}
