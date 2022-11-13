use std::fmt;

use async_trait::async_trait;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ObjectId {
    path: String,
    mime_type: String
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}

impl ObjectId {
    pub fn new(path: String, mime_type: String) -> Self {
        ObjectId { path, mime_type }
    }

    pub fn directory(path: String) -> Self {
        ObjectId { path, mime_type: String::from("directory") }
    }

    pub fn plain_text(path: String) -> Self {
        ObjectId { path, mime_type: String::from("text/plain") }
    }

    pub fn as_str(&self) -> &str {
        self.path.as_str()
    }

    pub fn mime_type(&self) -> String {
        self.mime_type.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct File {
    pub id: String,
    pub name: String,
    pub mime_type: Option<String>
}

pub struct Metadata {
    pub id: String,
    pub name: String,
    pub mime_type: String,
    pub open_path: String
}

#[async_trait]
pub trait FileSystem {
    async fn read_file(&self, object_id: ObjectId) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    async fn write_file(&self, object_id: ObjectId, content: Vec<u8>) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&self, object_id: ObjectId) -> Result<(), Box<dyn std::error::Error>>;
    async fn move_to(&self, object_id: ObjectId, new_parent_id: ObjectId) -> Result<(), Box<dyn std::error::Error>>;
    async fn rename(&self, object_id: ObjectId, new_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_folder_content(&self, object_id: ObjectId) -> Result<Vec<File>, Box<dyn std::error::Error>>;
    async fn create(&self, parent_id: ObjectId, file: File) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_metadata(&self, object_id: ObjectId) -> Result<Metadata, Box<dyn std::error::Error>>;
}