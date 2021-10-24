pub trait Easing {
    fn calc(&self, percent_complete: f64) -> f64;
}

pub trait Time {
    fn start(&mut self, now: f64);

    fn stop(&mut self, now: f64);

    fn reboot(&mut self, now: f64);
}

pub trait Elapsed: Time {
    fn get_elapsed_time(&self, now: f64) -> f64;
}
