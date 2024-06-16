use bytes::Bytes;
use tokio::{fs::File, io::AsyncWriteExt};

use std::path::PathBuf;

pub struct FileManager {}

impl FileManager {

    pub async fn save_file<'a>(name: &str, path: &str, data:Box<Bytes>) -> Result<Box<String>, Box<dyn std::error::Error>> {
        let mut file_path = PathBuf::from(path);
        file_path.push(name);

        let file_path_string = file_path
                        .as_path()
                        .to_str()
                        .unwrap()
                        .to_string();

        let file = File::create(file_path).await;
        match file {
            Ok(mut value) => {
                let write = value.write_all(&data).await;
                match  write {
                    Ok(_) => {
                        return Ok(Box::new(file_path_string));
                    }
                    Err(error) => {
                        return Err(Box::new(error));
                    }
                }
            }
            Err(error) => {
                return Err(Box::new(error));
            }
        }
    }

}