use crate::timer::{animation_timer::AnimationTimer, base_timer::Timer, model::Elapsed};

pub type Transducer = fn(f64) -> f64;

pub fn default_transducer(elapsed_time: f64) -> f64 {
    elapsed_time
}

pub struct TimeSystem {
    pub game_time: f64,
    pub last_time_transducer_was_set: f64,
    pub timer: Box<dyn Elapsed>,
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
        self.game_time = self.last_time_transducer_was_set
            + (self.transducer)(self.timer.get_elapsed_time(Timer::get_current_time()));
        self.reset();

        return self.game_time;
    }

    pub fn start(&mut self) {
        self.timer.start(Timer::get_current_time());
    }

    pub fn reset(&mut self) {
        self.timer.reboot(Timer::get_current_time());
        self.last_time_transducer_was_set = self.game_time;
    }

    pub fn _set_transducer(&mut self, transducer: Transducer, duration: Option<u16>) {
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
