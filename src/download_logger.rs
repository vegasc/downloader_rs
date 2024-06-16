use std::fmt::Write;

use crate::progress_logger_trait::ProgressLogger;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

pub struct DownloadLogger {
    pub(self) pb: Option<ProgressBar>
}

impl DownloadLogger {

    pub const fn new() -> DownloadLogger {
        return DownloadLogger { pb: None };
    }

}

impl ProgressLogger for DownloadLogger {

    fn log(&mut self, total: f64, current: f64) {
        match &self.pb {
            None => {
                let bar = ProgressBar::new(total as u64);
                bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                    .unwrap()
                    .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
                    .progress_chars("#>-"));
                self.pb = Some(bar);
            }
            Some(pb) => {
                if current >= total {
                    pb.finish();
                    return;
                }
                pb.set_position(current as u64);
            }
        }
    }

}