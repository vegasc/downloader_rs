use crate::progress_logger_trait::ProgressLogger;
use indicatif::{ProgressBar, ProgressStyle};

pub struct DownloadLogger {
    pub pb: Option<ProgressBar>,
}

impl DownloadLogger {
    pub const fn new() -> DownloadLogger {
        DownloadLogger { pb: None }
    }
}

impl ProgressLogger for DownloadLogger {
    fn log(&mut self, total: f64, current: f64) {
        match &mut self.pb {
            None => {
                let bar = ProgressBar::new(total as u64);
                bar.set_style(
                    ProgressStyle::default_bar()
                        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                        .unwrap()
                        .progress_chars("#>-"),
                );
                self.pb = Some(bar);
            }
            Some(pb) => {
                if current >= total {
                    pb.finish();
                } else {
                    pb.set_position(current as u64); // Update the current position
                }
            }
        }
    }
}