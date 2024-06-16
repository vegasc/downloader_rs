use bytes::Bytes;
use download_logger::DownloadLogger;
use reqwest::Url;
use std::env;
use std::process::Command;

mod inputer;
mod downloader;
mod file_namager;
mod progress_logger_trait;
mod download_logger;

use crate::inputer::Inputer;
use crate::downloader::Downloader;
use crate::file_namager::FileManager;

static DEST_PATH: &str = "/Users/aleksey/Documents/downloader/files_rs";

#[tokio::main]
async fn main() {
    process().await;
}

async fn process() {
    let regex = String::from(r"[-a-zA-Z0-9@:%_\+.~#?&//=]{2,256}\.[a-z]{2,4}\b(\/[-a-zA-Z0-9@:%_\+.~#?&//=]*)?");
    let inputer = Inputer::new(regex);

    let result: Result<String, String> = inputer
    .input("url: ", "error getting url");

    match result {
        Ok(value) => {
            download(&value).await;
        }
        Err(error) => {
            println!("{error}");
            return;
        }
    }
}

async fn download(url: &String) {
    let logger = Box::new(DownloadLogger::new());
    let result = Downloader::download(
        &url,
        logger
    ).await;
    match result {
        Ok(response) => {
            save_file(&url, response).await;
        }
        Err(error) => {
            println!("{error}");
        }
    }
}

async fn save_file(url: &String, data: Box<Bytes>) {
    let url_parsed = Url::parse(&url.to_string()).unwrap();
        
    let file_name = url_parsed
    .path_segments()
    .and_then(|segments| segments.last())
    .ok_or("file")
    .unwrap();

    let result = FileManager::save_file(file_name, DEST_PATH, data).await;
    match result {
        Ok(path) => {
            println!("File has been downloaded and saved:");
            println!("Path: {path}");
            open(path.as_str());
        }
        Err(error) => {
            println!("Error: {error}");
        }
    }

}

fn open(path: &str) {
    let os = env::consts::OS;
    if os == "macos" {
        Command::new("open")
        .arg("-R")
        .arg(path)
        .spawn()
        .unwrap();
    }
}