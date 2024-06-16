use bytes::Bytes;
use tokio::{fs::File, io::AsyncWriteExt};

pub struct FileManager {}

impl FileManager {

    pub async fn save_file<'a>(name: &str, path: &'a str, data:Box<Bytes>) -> Result<&'a str, Box<dyn std::error::Error>> {
        let file = File::create(name).await;
        match file {
            Ok(mut value) => {
                let write = value.write_all(&data).await;
                match  write {
                    Ok(_) => {
                        return Ok(path);
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