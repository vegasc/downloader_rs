use bytes::Bytes;
use futures_util::StreamExt;

use crate::progress_logger_trait::ProgressLogger;

// icon: 
// https://cdn-icons-png.flaticon.com/256/1160/1160358.png

// mp4(small):
// 

pub struct Downloader { }

impl Downloader {

    pub async fn download(url: String, mut logger: Box<dyn ProgressLogger>) -> Result<Box<Bytes>, Box<dyn std::error::Error>> {
        let response = reqwest::get(url).await?;

        let total_size = response
            .content_length()
            .ok_or("Failed to get content length")? as f64;

        let mut stream = response.bytes_stream();
        let mut downloaded: Vec<u8> = Vec::new();
        let mut downloaded_size = 0;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            downloaded_size += chunk.len();
            downloaded.extend_from_slice(&chunk);

            let progress = downloaded_size as f64 / total_size as f64 * 100.0;
            logger.log(total_size, progress);
        }

        Ok(Box::new(Bytes::from(downloaded)))
    }

}