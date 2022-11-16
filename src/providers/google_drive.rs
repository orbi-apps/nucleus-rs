// extern crate hyper;
// extern crate hyper_rustls;
extern crate google_drive3 as drive3;
use async_trait::async_trait;
use drive3::api::File as GoogleDriveFile;
use drive3::oauth2::storage::{TokenStorage, TokenInfo};
use serde_json::json;

use std::future::Future;
use std::pin::Pin;
use crate::interfaces::filesystem::{FileSystem, ObjectId, File, Metadata, self};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use drive3::{DriveHub, oauth2, hyper, hyper_rustls};
use drive3::oauth2::authenticator_delegate::{DefaultInstalledFlowDelegate, InstalledFlowDelegate};

pub struct GoogleDrive {
    hub: DriveHub<HttpsConnector<HttpConnector>>,
}

async fn browser_user_url(url: &str, need_code: bool) -> Result<String, String> {
    open::that(url).expect("An error occurred when trying to open web browser");
    let def_delegate = DefaultInstalledFlowDelegate;
    def_delegate.present_user_url(url, need_code).await
}

#[derive(Copy, Clone)]
struct InstalledFlowBrowserDelegate;

impl InstalledFlowDelegate for InstalledFlowBrowserDelegate {
    fn present_user_url<'a>(
        &'a self,
        url: &'a str,
        need_code: bool,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + 'a>> {
        Box::pin(browser_user_url(url, need_code))
    }
}

struct TokenStorageStrategy {
    pub path: String
}

#[async_trait]
impl TokenStorage for TokenStorageStrategy {
    async fn set(&self, scopes: &[&str], token: TokenInfo) -> anyhow::Result<()> {
        let path = self.path.clone() + scopes.join(".").replace("/", "0x2F").replace(":", "0x3A").as_str();
        println!("Writing token to {}", path);
        let contents = json!(&token);
        std::fs::write(path, contents.to_string())?;
        Ok(())
    }

    async fn get(&self, target_scopes: &[&str]) -> Option<TokenInfo> {
        let path = self.path.clone() + target_scopes.join(".").replace("/", "0x2F").replace(":", "0x3A").as_str();
        println!("Fetching token from {}", path);

        let contents_result = std::fs::read(path);
        if contents_result.is_err() { return None }

        let contents = contents_result.unwrap();

        let token_str = std::str::from_utf8(&contents);
        if token_str.is_err() { return None }

        let token: TokenInfo = serde_json::from_str(token_str.unwrap()).expect("Unable to serialize google drive tokens");
        
        Some(token)
    }
}

impl GoogleDrive {
    pub async fn new(path: String, client_secret: String) -> Result<GoogleDrive, Box<dyn std::error::Error>> {
        let secret = oauth2::parse_application_secret(client_secret.as_str()).unwrap();

        let storage = Box::new(TokenStorageStrategy { path });

        let auth = oauth2::InstalledFlowAuthenticator::builder(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect
        ).with_storage(storage).flow_delegate(Box::new(InstalledFlowBrowserDelegate)).build().await.unwrap();

        let hub = DriveHub::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()),
                auth);

        Ok(GoogleDrive { hub })
    }
}

impl From<google_drive3::api::File> for filesystem::File {
    fn from(file: GoogleDriveFile) -> Self {
        File { id: file.id.unwrap(), name: file.name.unwrap(), mime_type: file.mime_type }
    }
}

#[async_trait]
impl FileSystem for GoogleDrive {
    async fn read_file(&self, object_id: ObjectId) -> Result<Vec<u8>, Box<dyn std::error::Error>>
    {
        todo!()
    }

    async fn write_file(&self, object_id: ObjectId, content: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn delete(&self, object_id: ObjectId) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn rename(&self, object_id: ObjectId, new_name: String) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn move_to(&self, object_id: ObjectId, new_parent_id: ObjectId) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn create(&self, parent_id: ObjectId, file: File) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn list_folder_content(&self, object_id: ObjectId) -> Result<Vec<File>, Box<dyn std::error::Error>> {
        let id = if object_id.to_string() == "".to_string() {"root".to_string()} else {object_id.to_string()};
        let response = self.hub.files().list().q(format!("'{}' in parents", id).as_str()).doit().await?;
        
        let files: Vec<File> = response.1.files.unwrap().iter().map(|file| file.to_owned().into()).collect();
        
        Ok(files)
    }

    async fn get_metadata(&self, object_id: ObjectId) -> Result<Metadata, Box<dyn std::error::Error>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::drive3::oauth2;

    use crate::interfaces::filesystem::{FileSystem, ObjectId};

    use super::GoogleDrive;

    #[tokio::test]
    async fn connect_and_list_files() {
        let secret = json!(oauth2::read_application_secret("./sandbox/client.json").await.unwrap()).to_string();
        let drive = GoogleDrive::new("./sandbox/google-drive/".to_string(), secret).await.unwrap();
        let object_id = ObjectId::new("root".to_string(), "unknown".to_string());
        let result = drive.list_folder_content(object_id).await.unwrap();
        dbg!(result);
    }
}