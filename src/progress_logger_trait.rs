pub trait ProgressLogger {

    fn log(&mut self, total: f64, current: f64);

}