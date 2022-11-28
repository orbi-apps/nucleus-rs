use async_trait::async_trait;

use crate::{interfaces::filesystem::{FileSystem, ObjectId, File}, providers::onedrive::OneDrive};

use oauth2::TokenResponse;

use onedrive_api::{OneDrive as OneDriveApi, DriveLocation, ItemId, ItemLocation};

#[async_trait]
impl FileSystem for OneDrive {
    async fn read_file(&self, object_id: ObjectId) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn write_file(&self, object_id: ObjectId, content: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn delete(&self, object_id: ObjectId) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn move_to(&self, object_id: ObjectId, new_parent_id: ObjectId) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn rename(&self, object_id: ObjectId, new_name: String) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn list_folder_content(&self, object_id: ObjectId) -> Result<Vec<File>, Box<dyn std::error::Error>> {
        let drive = OneDriveApi::new(
            self.token.clone().unwrap().access_token().secret(), // Login token to Microsoft Graph.
            DriveLocation::me(),
        );

        let item_id : ItemId = object_id.clone().into();

        let item_location = if object_id.to_string() == "".to_string() { ItemLocation::root() } else { ItemLocation::from_id(&item_id) };

        let items = drive.list_children(item_location).await.unwrap();

        let files: Vec<File> = items.iter().map(|file| file.to_owned().into()).collect();

        Ok(files)
    }

    async fn create(&self, parent_id: ObjectId, file: File) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn get_metadata(&self, object_id: ObjectId) -> Result<crate::interfaces::filesystem::Metadata, Box<dyn std::error::Error>> {
        todo!()
    }
}
