use serde::Serialize;

use crate::models::{RegisterRequest, RegisterResponse, SignInVerifyRequest, SignInVerifyResponse};

pub struct PasswordlessClient {
    api_secret: String,
    client: reqwest::Client,
    base_url: String,
}

impl PasswordlessClient {
    pub fn new(api_secret: &str, base_url: &str) -> Self {
        return Self {
            api_secret: api_secret.to_owned(),
            client: reqwest::Client::new(),
            base_url: base_url.to_owned(),
        };
    }

    pub async fn register_token(
        &self,
        register_request: &RegisterRequest,
    ) -> Result<RegisterResponse, reqwest::Error> {
        let result: RegisterResponse = self
            .send_request(register_request, "register/token")
            .await?
            .json()
            .await?;

        return Ok(result);
    }

    pub async fn sign_in(
        &self,
        register_request: &SignInVerifyRequest,
    ) -> Result<SignInVerifyResponse, reqwest::Error> {
        let result: SignInVerifyResponse = self
            .send_request(register_request, "signin/verify")
            .await?
            .json()
            .await?;

        return Ok(result);
    }

    async fn send_request<T: Serialize>(
        &self,
        request: &T,
        path: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}", &self.base_url, path);
        let response = self
            .client
            .post(&url)
            .header("ApiSecret", &self.api_secret)
            .json(request)
            .send()
            .await?;

        Ok(response)
    }
}
