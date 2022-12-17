mod auth;
mod token;
mod interfaces;

extern crate google_drive3 as drive3;

use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use drive3::{DriveHub, hyper, hyper_rustls, oauth2::storage::TokenInfo};

pub type Token = TokenInfo;

pub struct GoogleDrive {
    hub: DriveHub<HttpsConnector<HttpConnector>>,
    tokens: token::MtTokenMap,
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;
    use super::drive3::oauth2;

    use crate::interfaces::filesystem::{FileSystem, ObjectId};

    use super::GoogleDrive;

    #[tokio::test]
    async fn connect_and_list_files() {
        let secret = json!(oauth2::read_application_secret("./sandbox/client.json").await.unwrap()).to_string();
        let drive = GoogleDrive::new(secret, HashMap::new()).await.unwrap();
        let object_id = ObjectId::new("root".to_string(), None);
        let result = drive.list_folder_content(object_id).await.unwrap();
        dbg!(result);
    }
}