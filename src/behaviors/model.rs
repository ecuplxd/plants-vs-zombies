use serde::Deserialize;

use crate::sprites::Velocit;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Horizontal
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum BehaviorType {
    Hover,
    Cycle,
    Walk,
    Switch,
    Frequency,
    Click,
    Scroll,
    ZombieCollision,
    PlantCollision,
    Drag,
    Interval,
}

impl Default for BehaviorType {
    fn default() -> BehaviorType {
        BehaviorType::Cycle
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BehaviorData {
    pub name: BehaviorType,
    #[serde(default)]
    pub duration: f64,
    #[serde(default)]
    pub interval: Option<f64>,
    #[serde(default)]
    pub rate: f64,
    #[serde(default)]
    pub distance: Option<f64>,

    #[serde(default = "default_infinite")]
    pub infinite: bool,
    #[serde(default)]
    pub switch_cells: Vec<String>,
    #[serde(default)]
    pub direction: Direction,
    #[serde(default)]
    pub delay: f64,
    #[serde(default)]
    pub velocit: Velocit,
}

fn default_infinite() -> bool {
    true
}
