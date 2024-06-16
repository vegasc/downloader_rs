use bytes::Bytes;
use tokio::{fs::File, io::AsyncWriteExt};

use std::path::PathBuf;

pub struct FileManager {}

impl FileManager {

    pub async fn save_file(name: &str, path: &str, data:Box<Bytes>) -> Result<String, Box<dyn std::error::Error>> {
        let mut file_path = PathBuf::from(path);
        file_path.push(name);

        let file_path_string = file_path
                        .as_path()
                        .to_str()
                        .unwrap()
                        .to_string();

        let mut file = File::create(file_path).await?;
        file.write_all(&data).await?;
        
        Ok(file_path_string)
    }

}