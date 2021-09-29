use crate::timer::{AnimationTimer, Elapsed, Timer};

pub type Transducer = fn(f64) -> f64;

pub fn default_transducer(elapsed_time: f64) -> f64 {
    elapsed_time
}

pub struct TimeSystem {
    game_time: f64,
    last_time_transducer_was_set: f64,
    timer: Box<dyn Elapsed>,
    transducer: Transducer,
}

impl TimeSystem {
    pub fn new() -> TimeSystem {
        TimeSystem {
            game_time: 0.0,
            last_time_transducer_was_set: 0.0,
            timer: Box::new(AnimationTimer::new(1000.0)),
            transducer: default_transducer,
        }
    }

    pub fn calculate_game_time(&mut self) -> f64 {
        let now = Timer::get_current_time();
        let transducer = (self.transducer)(self.timer.get_elapsed_time(now));

        self.game_time = self.last_time_transducer_was_set + transducer;
        self.reset();

        self.game_time
    }

    pub fn start(&mut self) {
        self.timer.start(Timer::get_current_time());
    }

    fn reset(&mut self) {
        self.timer.reboot(Timer::get_current_time());
        self.last_time_transducer_was_set = self.game_time;
    }

    fn _set_transducer(&mut self, transducer: Transducer, duration: Option<u16>) {
        let _last_transducer = self.transducer;

        self.calculate_game_time();
        self.reset();
        self.transducer = transducer;

        if let Some(_duration) = duration {
            todo!();
            // self.set_transducer(transducer, None);
        }
    }
}
