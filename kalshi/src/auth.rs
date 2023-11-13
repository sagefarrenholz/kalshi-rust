//! This module contains methods related to authentication. Logging in and Logging out.
use crate::kalshi_error::*;
use super::Kalshi;
use serde::{Deserialize, Serialize};



impl<'a> Kalshi<'a> {
    /// This is a method in the auth module
    pub async fn login(&mut self, user: &str, password: &str) -> Result<(), KalshiError> {
        let login_url: &str = &format!("{}/login", self.base_url.to_string());

        let login_payload = LoginPayload {
            email: user.to_string(),
            password: password.to_string(),
        };

        let result: LoginResponse = self
            .client
            .post(login_url)
            .json(&login_payload)
            .send()
            .await?
            .json()
            .await?;

        self.curr_token = Some(format!("Bearer {}", result.token));
        self.member_id = Some(result.member_id);

        return Ok(());
    }

    /// This is a method in the auth module
    pub async fn logout(&self) -> Result<(), KalshiError> {
        let logout_url: &str = &format!("{}/logout", self.base_url.to_string());

        self.client
            .post(logout_url)
            .header("Authorization", self.curr_token.clone().unwrap())
            .header("content-type", "application/json".to_string())
            .send()
            .await?;

        return Ok(());
    }
}

// used in login method
#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    member_id: String,
    token: String,
}
// used in login method
#[derive(Debug, Serialize, Deserialize)]
struct LoginPayload {
    email: String,
    password: String,
}
