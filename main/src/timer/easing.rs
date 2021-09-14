use super::model::Easing;

pub struct EaseOut {
    pub strength: f64,
}

impl EaseOut {
    pub fn _new() -> EaseOut {
        EaseOut { strength: 1.0 }
    }
}

impl Easing for EaseOut {
    fn calc(&self, percent_complete: f64) -> f64 {
        return 1.0 - f64::powf(1.0 - percent_complete, self.strength * 2.0);
    }
}
