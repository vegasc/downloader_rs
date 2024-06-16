use bytes::Bytes;
use futures::FutureExt;
use futures_util::StreamExt;

pub struct Downloader {
    url: String
}

// icon sample
// https://cdn-icons-png.flaticon.com/256/1160/1160358.png



impl Downloader {

    pub fn new(url: String) -> Downloader {
        return Downloader { url:url };
    }
    
    pub async fn download(&self) -> Result<Box<Bytes>, Box<dyn std::error::Error>> {
        let stream = reqwest::get(&self.url).await?;

        let total_size = stream.content_length()
            .ok_or("Failed to get content length")?;

        let mut downloaded: Vec<u8> = Vec::new();
        let mut downloaded_size = 0;

        let mut stream_pin = Box::pin(stream.bytes().into_stream());

        while let Some(item) = stream_pin.next().await {
            let chunk = item?;
            downloaded_size += chunk.len();
            downloaded.extend_from_slice(&chunk);

            let progress = downloaded_size as f64 / total_size as f64 * 100.0;
            println!("Download progress: {:.2}%", progress);
        }

        Ok(Box::new(Bytes::from(downloaded)))
    }

}