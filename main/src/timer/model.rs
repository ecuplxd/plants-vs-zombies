use super::base_timer::Timer;

/* *************** trait *************** */

pub trait Elapsed: Time {
    fn get_elapsed_time(&self, now: f64) -> f64;
}

pub trait Easing {
    fn calc(&self, percent_complete: f64) -> f64;
}

pub trait Time {
    fn get_timer(&mut self) -> &mut Timer;

    fn start(&mut self, now: f64) {
        let timer = self.get_timer();

        timer.start(now);
    }

    fn stop(&mut self, now: f64) {
        let timer = self.get_timer();

        timer.stop(now);
    }

    fn reboot(&mut self, now: f64) {
        let timer = self.get_timer();

        timer.reboot(now);
    }

    fn is_working(&mut self) -> bool {
        let timer = self.get_timer();

        return timer.is_running() && !timer.is_paused();
    }
}
