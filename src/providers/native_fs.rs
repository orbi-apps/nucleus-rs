use async_trait::async_trait;
use eyre::Result;
use std::{fs, path::Path};
use serde::{Serialize, Deserialize};

use crate::interfaces::filesystem::{FileSystem, ObjectId, File, Metadata};


#[derive(Debug, Serialize, Deserialize)]
pub struct NativeFs {
    pub root: String,
}

impl NativeFs {
    pub fn new(root: String) -> NativeFs{
      NativeFs {
        root
      }
    }
}

#[async_trait]
impl FileSystem for NativeFs {
    async fn read_file(&self, object_id: ObjectId) -> Result<Vec<u8>, Box<dyn std::error::Error>>
    {
        let content = fs::read(self.root.clone() + object_id.as_str())?;

        Ok(content)
    }

    async fn write_file(&self, object_id: ObjectId, content: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(self.root.clone() + object_id.as_str(), content)?;
        Ok(())
    }

    async fn delete(&self, object_id: ObjectId) -> Result<(), Box<dyn std::error::Error>> {
        if object_id.mime_type() == "directory".to_string() {
            fs::remove_dir(self.root.clone() + object_id.as_str())?;
        } else {
            fs::remove_file(self.root.clone() + object_id.as_str())?;
        }
        Ok(())
    }

    async fn rename(&self, object_id: ObjectId, new_name: String) -> Result<(), Box<dyn std::error::Error>> {
        let new_path = std::path::Path::new(object_id.as_str()).parent().unwrap().join(new_name);
        fs::rename(self.root.clone() + object_id.as_str(), self.root.clone() + new_path.to_str().unwrap())?;
        Ok(())
    }

    async fn move_to(&self, object_id: ObjectId, new_parent_id: ObjectId) -> Result<(), Box<dyn std::error::Error>> {
        let object_id_split: Vec<&str> = object_id.as_str().split("/").collect();
        fs::rename(self.root.clone() + object_id.as_str(), self.root.clone() + new_parent_id.as_str() + "/" + object_id_split[object_id_split.len() - 1])?;
        Ok(())
    }

    async fn create(&self, parent_id: ObjectId, file: File) -> Result<(), Box<dyn std::error::Error>> {
        if file.mime_type == Some("directory".to_string()) {
            fs::create_dir(self.root.clone() + parent_id.as_str() + "/" + file.name.as_str())?;
        } else {
            let object_id = ObjectId::new(self.root.clone() + parent_id.as_str() + "/" + file.name.as_str(), parent_id.mime_type());
            self.write_file(object_id, vec![]).await?;
        }
        Ok(())
    }

    async fn list_folder_content(&self, object_id: ObjectId) -> Result<Vec<File>, Box<dyn std::error::Error>> {
        let dir_content = fs::read_dir(self.root.clone() + object_id.as_str())?;

        let mut files = vec![];

        for file in dir_content {
            let entry = file.unwrap();
            let full_path = entry.path().as_os_str().to_str().unwrap().to_string();
            files.push(File {
                id: full_path.strip_prefix(&self.root.clone()).unwrap().to_string(),
                name: entry.file_name().to_string_lossy().to_string(),
                mime_type: if entry.metadata().unwrap().is_dir() {
                    Some("directory".to_string())
                } else if entry.metadata().unwrap().is_symlink() {
                    Some("symlink".to_string())
                } else {
                    Some("text/plain".to_string())
                }
            });
        }
        Ok(files)
    }

    async fn get_metadata(&self, object_id: ObjectId) -> Result<crate::interfaces::filesystem::Metadata, Box<dyn std::error::Error>> {
        let metadata = std::fs::metadata(self.root.clone() + object_id.as_str()).unwrap();
        Ok(Metadata {
            id: object_id.to_string(),
            name: Path::new(object_id.as_str()).file_name().unwrap().to_str().unwrap().to_string(),
            mime_type: if metadata.is_dir() { "directory".to_string() } else { "".to_string() },
            open_path: self.root.clone() + object_id.as_str()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::providers::native_fs::*;
    use crate::interfaces::filesystem::FileSystem;

    #[tokio::test]
    async fn native_fs_request_works() {
        let x = NativeFs {
            root: "./sandbox/".to_string()
        };
        let object_id = ObjectId::new(String::from("hello-world.txt"), String::from("text/plain"));
        let result = x.read_file(object_id).await;
        assert!(result.is_ok());
        assert_eq!(String::from_utf8(result.unwrap().to_vec()).unwrap(), String::from("hello world!"));
    }

    #[tokio::test]
    async fn native_fs_list_folder_content() {
        let x = NativeFs {
            root: "./sandbox/".to_string()
        };

        let object_id = ObjectId::new(String::from(""), String::from("directory"));

        let result = x.list_folder_content(object_id).await;

        assert!(result.is_ok());

        assert_eq!("hello-world.txt", result.as_ref().unwrap()[0].name);
    }
}