use bytes::Bytes;
use futures_util::StreamExt;

use crate::progress_logger_trait::ProgressLogger;

pub struct Downloader { }

impl Downloader {

    pub async fn download(url: String, mut logger: Box<dyn ProgressLogger>) -> Result<Box<Bytes>, Box<dyn std::error::Error>> {
        let response = reqwest::get(url).await?;

        let total_size = response
            .content_length()
            .unwrap_or_default() as f64;

        let mut stream = response.bytes_stream();
        let mut downloaded: Vec<u8> = Vec::new();
        let mut downloaded_size = 0;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            downloaded_size += chunk.len();
            downloaded.extend_from_slice(&chunk);

            logger.log(total_size, downloaded_size as f64);
        }

        Ok(Box::new(Bytes::from(downloaded)))
    }

}