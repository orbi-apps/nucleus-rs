use crate::interfaces::filesystem::{FileSystem, ObjectId, File, Metadata};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleDrive {
    access_token: String
}

impl GoogleDrive {
    pub fn new(access_token: String) -> Result<GoogleDrive, Box<dyn std::error::Error>> {
        Ok(GoogleDrive { access_token })
    }
}

impl FileSystem for GoogleDrive {
    fn read_file(&self, object_id: ObjectId) -> Result<Vec<u8>, Box<dyn std::error::Error>>
    {
        let client = reqwest::blocking::Client::new();

        let res = client.get(
            "https://www.googleapis.com/drive/v3/files/".to_string() +
            object_id.as_str() + "?alt=media").header(
                "Authorization", 
                "Bearer ".to_string() + self.access_token.as_str()).send()?;

        Ok(res.bytes().unwrap().to_vec())
    }

    fn write_file(&self, object_id: ObjectId, content: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn delete(&self, object_id: ObjectId) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn rename(&self, object_id: ObjectId, new_name: String) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn move_to(&self, object_id: ObjectId, new_parent_id: ObjectId) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn create(&self, parent_id: ObjectId, file: File) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn list_folder_content(&self, object_id: ObjectId) -> Result<Vec<File>, Box<dyn std::error::Error>> {
        let file = File {
            id: String::from(""),
            name: object_id.to_string(),
            mime_type: "file".to_string()
        };
        Ok(vec![file])
    }

    fn get_metadata(&self, object_id: ObjectId) -> Result<Metadata, Box<dyn std::error::Error>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::providers::google_drive;
    use crate::interfaces::filesystem::{FileSystem, ObjectId};
    use crate::read_token;

    #[test]
    fn oauth_request() {
        let token = read_token();
        let google_client = google_drive::GoogleDrive::new(token);
        let file_id = ObjectId::new("1YGx_HIdACC78G-eb7ydVoqyRixg_JNlC".to_string(), "text/plain".to_string());
        assert!(google_client.is_ok());
        let file = google_client.unwrap().read_file(file_id);
        assert!(file.is_ok());
    }
}