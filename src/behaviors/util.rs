use super::{BehaviorData, CycleBehavior, IntervalBehavior};
use crate::behaviors::collision::CollisionBehavior;
use crate::behaviors::{
    Behavior, BehaviorType, ClickBehavior, DragBehavior, FrequencyBehavior, HoverBehavior,
    ScrollBehavior, SwitchBehavior, WalkBehavior,
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
            BehaviorType::Hover => Box::new(HoverBehavior::new()),
            BehaviorType::Cycle => Box::new(CycleBehavior::new(*duration, *interval)),
            BehaviorType::Walk => Box::new(WalkBehavior::new(*velocit, *duration, *distance)),
            BehaviorType::Switch => Box::new(SwitchBehavior::new(
                BehaviorFactory::get_switch_cells(resource, switch_cells, sheet_kind),
                *duration,
                *infinite,
            )),
            BehaviorType::Frequency => Box::new(FrequencyBehavior::new(*duration, *delay)),
            BehaviorType::Click => Box::new(ClickBehavior::new()),
            BehaviorType::Scroll => Box::new(ScrollBehavior::new(distance.unwrap(), *rate)),
            BehaviorType::Collision => Box::new(CollisionBehavior::new()),
            BehaviorType::Drag => Box::new(DragBehavior::new()),
            BehaviorType::Interval => Box::new(IntervalBehavior::new(interval.unwrap())),
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
