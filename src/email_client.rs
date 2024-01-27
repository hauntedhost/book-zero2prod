use crate::domain::SubscriberEmail;
use reqwest::Client;
use secrecy::Secret;

pub struct EmailClient {
    http_client: Client,
    base_url: String,
    auth_token: Secret<String>,
    sender: SubscriberEmail,
}

impl EmailClient {
    pub fn new(base_url: String, auth_token: Secret<String>, sender: SubscriberEmail) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            auth_token,
            sender,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        todo!()
    }
}
