use crate::behaviors::model::Behavior;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
pub struct BehaviorAction {
    pub(crate) behavior: Behavior,
    pub(crate) node_id: u32,
    pub(crate) last_performed: u128,
}

impl Ord for BehaviorAction {
    fn cmp(&self, other: &Self) -> Ordering {
        other.last_performed.cmp(&self.last_performed)
    }
}

impl PartialOrd for BehaviorAction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct ActionResult {
    pub enemy_position: (f32, f32),
    pub enemy_target: (f32, f32),
}
