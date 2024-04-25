use crate::behaviors::model::{Behavior, BehaviorTreeTrait, NodeTrait};
use rand::Rng;
use std::cmp::Ordering;

pub trait EnemyAi {
    fn new() -> Self
    where
        Self: Sized;
    fn perform_action(
        &mut self,
        current_time: u128,
        player_position: (f32, f32),
        enemy_position: (f32, f32),
        speed: f32,
        projectile_positions: Vec<(f32, f32)>,
    ) -> Result<ActionResult, anyhow::Error>;
}

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
    pub is_attacking: bool,
}

pub fn get_next_child_actions(
    behavior_tree: &dyn BehaviorTreeTrait,
    current_action: &BehaviorAction,
    current_time: u128,
) -> Vec<BehaviorAction> {
    let mut actions = Vec::new();
    if let Some((left, right)) = behavior_tree.get_node_children(current_action.node_id) {
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

pub fn calculate_dodge_position(
    enemy_position: (f32, f32),
    projectile_positions: Vec<(f32, f32)>,
    speed: f32,
) -> (f32, f32) {
    // check if there are any projectiles nearby
    let mut nearby_projectiles = Vec::new();
    for projectile_position in projectile_positions {
        let dx = projectile_position.0 - enemy_position.0;
        let dy = projectile_position.1 - enemy_position.1;
        let distance = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
        if distance < 50.0 {
            nearby_projectiles.push(projectile_position);
        }
    }

    // if there are nearby projectiles, dodge them
    if !nearby_projectiles.is_empty() {
        // get the closest projectile
        nearby_projectiles.sort_by(|a, b| {
            let dx_a = a.0 - enemy_position.0;
            let dy_a = a.1 - enemy_position.1;
            let distance_a = (dx_a.powf(2.0) + dy_a.powf(2.0)).sqrt();
            let dx_b = b.0 - enemy_position.0;
            let dy_b = b.1 - enemy_position.1;
            let distance_b = (dx_b.powf(2.0) + dy_b.powf(2.0)).sqrt();
            distance_a.partial_cmp(&distance_b).unwrap()
        });

        let mut dodge_direction = (0.0, 0.0);

        // Calculate the direction to dodge the nearest projectile
        let closest_projectile = nearby_projectiles[0];
        let dx = closest_projectile.0 - enemy_position.0;
        let dy = closest_projectile.1 - enemy_position.1;
        let magnitude = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
        dodge_direction = (dx / magnitude, dy / magnitude);
        // Normalize the dodge direction
        let magnitude = (dodge_direction.0.powf(2.0) + dodge_direction.1.powf(2.0)).sqrt();
        let direction = (dodge_direction.0 / magnitude, dodge_direction.1 / magnitude);

        return (
            enemy_position.0 - direction.0 * speed,
            enemy_position.1 - direction.1 * speed,
        );
    } else {
        // If there are no nearby projectiles, move randomly
        // Generate a random angle and distance
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
        let distance = rng.gen_range(0.0..500.0); // Set this to the desired dodge distance

        // Calculate the new position
        let dx = distance * angle.cos();
        let dy = distance * angle.sin();

        return (enemy_position.0 + dx, enemy_position.1 + dy);
    }
}
