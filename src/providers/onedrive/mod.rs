mod interfaces;
mod auth;
pub mod token;

use onedrive_api::{ItemId, resource::DriveItem};

use crate::interfaces::filesystem::{ObjectId, File};

use self::token::TokenStorage;

pub struct OneDrive {
    token: TokenStorage,
    client_id: String
}

impl From<ObjectId> for ItemId {
    fn from(object_id: ObjectId) -> Self {
        ItemId(object_id.to_string())
    }
}

impl From<DriveItem> for File {
    fn from(item: DriveItem) -> Self {
        File {
            id: item.id.unwrap().as_str().to_string(),
            name: item.name.unwrap(),
            mime_type: if item.folder.is_some() { Some("directory".to_string()) } else { None }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{providers::onedrive::*, interfaces::filesystem::FileSystem};

    #[tokio::test]
    async fn one_drive_login_works() {
        let client_id_vec = std::fs::read("./sandbox/onedrive").unwrap();
        let client_id = std::str::from_utf8(&client_id_vec).unwrap();
        let onedrive = OneDrive::new(None, client_id.to_string());
        onedrive.fetch_credentials().await.unwrap();
        println!("{:?}", onedrive.token.get().await);
    }

    #[tokio::test]
    async fn one_drive_upload_works() {
        let client_id_vec = std::fs::read("./sandbox/onedrive").unwrap();
        let client_id = std::str::from_utf8(&client_id_vec).unwrap();
        let onedrive = OneDrive::new(None, client_id.to_string());
        onedrive.fetch_credentials().await.unwrap();
        
        onedrive.write_file(ObjectId::new(
            "5EE7F7AB27F9809D!120".to_string(),
            Some("text/plain".to_string())),
            "hello world lakjdlsa kjd ijda dlisajd qadioj qwoidj oaidjq oijdqoi djqwdj"
            .as_bytes().to_vec()).await.unwrap();
    }
}