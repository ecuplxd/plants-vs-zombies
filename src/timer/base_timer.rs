use super::{Elapsed, Time};
use crate::util::window;

#[derive(Debug, Default)]
pub struct Timer {
    elapsed: f64,
    paused: bool,
    running: bool,
    start_time: f64,
    start_pause: f64,
    total_paused_time: f64,
}

impl Timer {
    pub fn get_current_time() -> f64 {
        let performance = window()
            .performance()
            .expect("performance should be available");

        performance.now()
    }

    fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    fn pause(&mut self, now: f64) {
        if self.paused {
            return;
        }

        self.start_pause = now;
        self.paused = true
    }

    fn unpause(&mut self, now: f64) {
        if !self.paused {
            return;
        }

        self.total_paused_time += now - self.start_pause;
        self.start_pause = 0.0;
        self.paused = false;
    }

    fn reset(&mut self, now: f64) {
        self.elapsed = 0.0;
        self.paused = false;
        self.running = false;
        self.start_pause = 0.0;
        self.total_paused_time = 0.0;
        self.start_time = now;
    }
}

impl Time for Timer {
    fn start(&mut self, now: f64) {
        self.running = true;
        self.total_paused_time = 0.0;
        self.start_pause = 0.0;
        self.start_time = now;
    }

    fn stop(&mut self, now: f64) {
        if self.paused {
            self.unpause(now);
        }

        self.elapsed = now - self.start_time - self.total_paused_time;
        self.running = false;
    }

    fn reboot(&mut self, now: f64) {
        self.stop(now);
        self.reset(now);
        self.start(now);
    }
}

impl Elapsed for Timer {
    fn get_elapsed_time(&self, now: f64) -> f64 {
        match self.running {
            true => now - self.start_time - self.total_paused_time,
            false => self.elapsed,
        }
    }
}
