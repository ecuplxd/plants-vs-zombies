use super::{Easing, Elapsed, Time, Timer};

pub struct AnimationTimer {
    pub duration: f64,
    pub timer: Timer,
    easing: Option<Box<dyn Easing>>,
}

impl AnimationTimer {
    pub fn new(duration: f64) -> AnimationTimer {
        AnimationTimer {
            duration,
            timer: Default::default(),
            easing: None,
        }
    }

    pub fn is_expired(&self, now: f64) -> bool {
        self.get_elapsed_time(now) > self.duration
    }

    pub fn is_running(&self) -> bool {
        self.timer.is_running()
    }
}

impl Time for AnimationTimer {
    fn start(&mut self, now: f64) {
        self.timer.start(now);
    }

    fn stop(&mut self, now: f64) {
        self.timer.stop(now);
    }

    fn reboot(&mut self, now: f64) {
        self.timer.reboot(now);
    }
}

impl Elapsed for AnimationTimer {
    fn get_elapsed_time(&self, now: f64) -> f64 {
        let elapsed_time = self.timer.get_elapsed_time(now);
        let percent_complete = elapsed_time / self.duration;

        if percent_complete == 0.0 || percent_complete > 1.0 {
            return elapsed_time;
        }

        match &self.easing {
            Some(easing) => elapsed_time * (easing.calc(percent_complete) / percent_complete),
            None => elapsed_time,
        }
    }
}
