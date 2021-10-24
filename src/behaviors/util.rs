use super::{BehaviorData, Cycle, Interval};
use crate::behaviors::{
    Behavior, BehaviorType, Click, Drag, Frequency, Hover, PlantCollision, Scroll, Switch, Walk,
    ZombieCollision,
};
use crate::model::Resource;
use crate::sprites::SpriteCell;

pub struct BehaviorFactory;

impl BehaviorFactory {
    pub fn create(
        resource: &Resource,
        behavior_data: &BehaviorData,
        sheet_kind: &str,
    ) -> Box<dyn Behavior> {
        let BehaviorData {
            name,
            duration,
            interval,
            rate,
            distance,
            infinite,
            switch_cells,
            delay,
            velocit,
            ..
        } = behavior_data;

        let behavior: Box<dyn Behavior> = match name {
            BehaviorType::Click => Box::new(Click::new()),
            BehaviorType::Cycle => Box::new(Cycle::new(*duration, *interval)),
            BehaviorType::Drag => Box::new(Drag::new()),
            BehaviorType::Frequency => Box::new(Frequency::new(*duration, *delay)),
            BehaviorType::Hover => Box::new(Hover::new()),
            BehaviorType::Interval => Box::new(Interval::new(interval.unwrap())),
            BehaviorType::PlantCollision => Box::new(PlantCollision::new()),
            BehaviorType::Scroll => Box::new(Scroll::new(distance.unwrap(), *rate)),
            BehaviorType::Switch => Box::new(Switch::new(
                BehaviorFactory::get_switch_cells(resource, switch_cells, sheet_kind),
                *duration,
                *infinite,
            )),
            BehaviorType::Walk => Box::new(Walk::new(*velocit, *duration, *distance)),
            BehaviorType::ZombieCollision => Box::new(ZombieCollision::new()),
        };

        behavior
    }

    fn get_switch_cells(
        resource: &Resource,
        switch_cells: &[String],
        sheet_kind: &str,
    ) -> Vec<Vec<SpriteCell>> {
        let cells: Vec<Vec<SpriteCell>> = switch_cells
            .iter()
            .filter_map(|switch_cell| {
                resource
                    .get_cells_may_not_exit(&format!("{}/{}", sheet_kind, switch_cell))
                    .map(|cell| cell.to_vec())
            })
            .collect();

        cells
    }

    pub fn whether_to_enable(behavior: &mut Box<dyn Behavior>, now: f64) {
        let behavior_type = behavior.name();

        if behavior_type == BehaviorType::Click {
            behavior.stop(now);
        } else if behavior_type != BehaviorType::Interval {
            behavior.start(now)
        }
    }
}
