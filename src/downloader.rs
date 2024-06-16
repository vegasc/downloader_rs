use bytes::Bytes;
use reqwest::Client;

pub struct Downloader {
    url: String,
    pub(self) client: Client
}

impl Downloader {

    pub fn new(url: String) -> Downloader {
        return Downloader { url:url, client: Client::new() };
    }

    pub async fn download(&self) -> Result<Box<Bytes>, Box<dyn std::error::Error>> {
        let response = self.client
        .get(self.url.to_string())
        .send()
        .await;

        match response {
            Ok(value) => {
                let bytes: Bytes = value.bytes().await.unwrap();
                let bytes_box: Box<Bytes> = Box::new(bytes);
                return Ok(bytes_box);
            }
            Err(error) => {
                return Err(Box::new(error));
            }
        }
        
    }

}