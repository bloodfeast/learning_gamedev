use crate::behaviors::enemy_ai::model::{
    get_next_child_actions, ActionResult, BehaviorAction, EnemyAi,
};
use crate::behaviors::model::{Behavior, BehaviorTreeTrait, NodeTrait};
use crate::behaviors::normal_enemy_behavior_tree::NormalEnemyBehaviorTree;
use anyhow::Result;
use rand::Rng;
use std::collections::BinaryHeap;

pub(crate) struct NormalEnemyAI {
    behavior_tree: NormalEnemyBehaviorTree,
    current_action: BehaviorAction,
    action_heap: BinaryHeap<BehaviorAction>,
}

impl EnemyAi for NormalEnemyAI {
    fn new() -> Self {
        let behavior_tree = NormalEnemyBehaviorTree::new();
        let current_action = BehaviorAction {
            behavior: behavior_tree.get_root().get_name(),
            node_id: behavior_tree.get_root().get_id(),
            last_performed: 0,
        };
        let mut action_heap = BinaryHeap::new();
        // Push the root node into the heap
        action_heap.push(current_action.clone());

        // Push the children of the root node into the heap
        get_next_child_actions(&behavior_tree, &current_action, 0)
            .iter()
            .for_each(|action| {
                action_heap.push(action.clone());
            });

        NormalEnemyAI {
            behavior_tree,
            current_action,
            action_heap,
        }
    }

    fn perform_action(
        &mut self,
        current_time: u128,
        player_position: (f32, f32),
        enemy_position: (f32, f32),
        _speed: f32,
    ) -> Result<ActionResult, anyhow::Error> {
        let mut result = ActionResult {
            enemy_position,
            enemy_target: player_position,
            is_attacking: false,
        };
        if current_time - self.current_action.last_performed >= 1000 {
            // Perform the action
            match self.current_action.behavior {
                Behavior::Idle => {
                    result.enemy_position = enemy_position;
                }
                Behavior::MoveToRandom => {
                    let mut rng = rand::thread_rng();
                    let x = rng.gen_range(
                        (enemy_position.0 - 1000.0).min(0.0)
                            ..(enemy_position.0 + 1000.0).max(1920.0),
                    );
                    let y = rng.gen_range(
                        (enemy_position.1 - 1000.0).min(0.0)
                            ..(enemy_position.1 + 1000.0).max(1080.0),
                    );
                    result.enemy_position = (x, y);
                }
                Behavior::MoveToPlayer => {
                    let x = player_position.0;
                    let y = player_position.1;
                    result.enemy_position = (x, y);
                }
                Behavior::AttackRandom => {
                    let mut rng = rand::thread_rng();
                    let x = rng.gen_range(
                        (player_position.0 - 500.0).min(0.0)
                            ..(player_position.0 + 500.0).max(1920.0),
                    );
                    let y = rng.gen_range(
                        (player_position.1 - 500.0).min(0.0)
                            ..(player_position.1 + 500.0).max(1080.0),
                    );
                    result.enemy_target = (x, y);
                    result.is_attacking = true;
                }
                Behavior::AttackPlayer => {
                    let x = player_position.0;
                    let y = player_position.1;
                    result.enemy_target = (x, y);
                    result.is_attacking = true;
                }
                _ => {
                    result.enemy_position = enemy_position;
                }
            }

            if self
                .behavior_tree
                .get_node_children(self.current_action.node_id)
                .is_some()
                && self.current_action.last_performed == 0
            {
                // Push the children of the current action into the heap
                get_next_child_actions(&self.behavior_tree, &self.current_action, current_time)
                    .iter()
                    .for_each(|action| {
                        self.action_heap.push(action.clone());
                    });
            }

            // Update the last performed time
            self.current_action.last_performed = current_time;

            // Push the current action back into the heap
            self.action_heap.push(self.current_action.clone());

            // Pop the next action from the heap
            self.current_action = self.action_heap.pop().unwrap_or(BehaviorAction {
                behavior: Behavior::Idle,
                node_id: 0,
                last_performed: current_time,
            });
        }
        return Ok(result);
    }
}
