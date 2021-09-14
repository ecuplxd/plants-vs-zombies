#[derive(Debug, Default, Clone, Copy)]
pub struct ZombieState {
    pub switch_index: usize,
    pub waiting: bool,
    pub walking: bool,
    pub attacking: bool,
    pub dieing: bool,
    pub died: bool,
}

impl ZombieState {
    pub fn new() -> ZombieState {
        ZombieState {
            waiting: true,
            walking: true,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct ZombieData {
    pub life: f64,
    pub attack: f64,
}
