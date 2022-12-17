use oauth2::{basic::BasicTokenType, StandardTokenResponse, EmptyExtraTokenFields};
use tokio::sync::Mutex;

pub type OneDriveToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

pub struct TokenStorage {
    token: Mutex<Option<OneDriveToken>>
}

impl TokenStorage {
    pub async fn get(&self) -> Option<OneDriveToken> {
        (*self.token.lock().await).clone()
    }

    pub async fn set(&self, token: Option<OneDriveToken>) {
        let mut x = self.token.lock().await;
        *x = token;
    }

    pub fn new(token: Option<OneDriveToken>) -> Self {
        TokenStorage { token: Mutex::new(token) }
    }
}