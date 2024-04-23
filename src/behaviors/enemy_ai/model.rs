use crate::behaviors::model::{Behavior, BehaviorTreeTrait, NodeTrait};
use rand::Rng;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone, Debug)]
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

pub fn get_next_child_actions(
    behavior_tree: &dyn BehaviorTreeTrait,
    current_action: &BehaviorAction,
    current_time: u128,
) -> Vec<BehaviorAction> {
    let mut actions = Vec::new();
    if let Some((left, right)) = behavior_tree.get_next_behaviors(current_action.node_id) {
        let random = rand::thread_rng().gen_range(0..2);
        if random == 0 {
            if let Some(left_node) = behavior_tree.get_node(left) {
                actions.push(BehaviorAction {
                    behavior: left_node.get_name(),
                    node_id: left_node.get_id(),
                    last_performed: current_time,
                });
            }
            if let Some(right_node) = behavior_tree.get_node(right) {
                actions.push(BehaviorAction {
                    behavior: right_node.get_name(),
                    node_id: right_node.get_id(),
                    last_performed: current_time,
                });
            }
        } else {
            if let Some(right_node) = behavior_tree.get_node(right) {
                actions.push(BehaviorAction {
                    behavior: right_node.get_name(),
                    node_id: right_node.get_id(),
                    last_performed: current_time,
                });
            }
            if let Some(left_node) = behavior_tree.get_node(left) {
                actions.push(BehaviorAction {
                    behavior: left_node.get_name(),
                    node_id: left_node.get_id(),
                    last_performed: current_time,
                });
            }
        }
    }
    actions
}
