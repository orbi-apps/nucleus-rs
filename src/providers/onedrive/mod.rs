mod interfaces;
mod auth;

use oauth2::{basic::BasicTokenType, StandardTokenResponse, EmptyExtraTokenFields};
use onedrive_api::{ItemId, resource::DriveItem};
use serde::{Serialize, Deserialize};

use crate::interfaces::filesystem::{ObjectId, File};

type OneDriveToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

#[derive(Serialize, Deserialize)]
pub struct OneDrive {
    token: Option<OneDriveToken>,
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
    use crate::providers::onedrive::*;

    #[tokio::test]
    async fn one_drive_login_works() {
        let client_id_vec = std::fs::read("./sandbox/onedrive").unwrap();
        let client_id = std::str::from_utf8(&client_id_vec).unwrap();
        let mut onedrive = OneDrive::new(None, client_id.to_string());
        onedrive.fetch_credentials().await.unwrap();
        println!("{:?}", onedrive.token);
    }
}