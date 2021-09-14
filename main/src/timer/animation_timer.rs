use super::{
    base_timer::Timer,
    model::{Easing, Time},
    Elapsed,
};

pub struct AnimationTimer {
    pub duration: f64,
    pub timer: Timer,
    pub last_finished_time: f64,
    easing: Option<Box<dyn Easing>>,
}

impl AnimationTimer {
    pub fn new(duration: f64) -> AnimationTimer {
        AnimationTimer {
            duration,
            timer: Default::default(),
            last_finished_time: 0.0,
            easing: None,
        }
    }

    pub fn is_finished(&self, now: f64) -> bool {
        return now - self.last_finished_time > self.duration;
    }

    pub fn is_expired(&self, now: f64) -> bool {
        return self.get_elapsed_time(now) > self.duration;
    }

    pub fn finisehd(&mut self, now: f64) {
        self.last_finished_time = now;
    }
}

impl Time for AnimationTimer {
    fn get_timer(&mut self) -> &mut Timer {
        return &mut self.timer;
    }
}

impl Elapsed for AnimationTimer {
    fn get_elapsed_time(&self, now: f64) -> f64 {
        let elapsed_time = self.timer.get_elapsed_time(now);
        let percent_complete = elapsed_time / self.duration;

        if percent_complete == 0.0 || percent_complete > 1.0 {
            return elapsed_time;
        }

        return match &self.easing {
            Some(easing) => elapsed_time * (easing.calc(percent_complete) / percent_complete),
            None => elapsed_time,
        };
    }
}
