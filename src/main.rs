use std::borrow::Borrow;

use futures::executor::block_on;
use bytes::Bytes;

mod inputer;
mod downloader;

use crate::inputer::Inputer;
use crate::downloader::Downloader;

fn main() {
    block_on(process());
}

async fn process() {
    let regex = String::from(r"[-a-zA-Z0-9@:%_\+.~#?&//=]{2,256}\.[a-z]{2,4}\b(\/[-a-zA-Z0-9@:%_\+.~#?&//=]*)?");
    let inputer = Inputer::new(regex);

    let result: Result<String, String> = inputer
    .input(String::from("url: "), String::from("error getting url"));

    match result {
        Ok(value) => {
            println!("Value: {value}");
            download(value).await;
        }
        Err(error) => {
            println!("{error}");
            return;
        }
    }
}

async fn download(url: String) {
    let downloader = Downloader::new(url.clone());
    let result = downloader.download().await;
    match result {
        Ok(response) => {
            save_file(url, response);
        }
        Err(error) => {
            println!("{error}");
        }
    }
}

fn save_file(url: String, data: Box<Bytes>) {
    
}