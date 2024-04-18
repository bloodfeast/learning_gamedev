use crate::actors::models::{Actor, ActorType};
use ggez::graphics::{Color, Mesh};

pub fn create_enemy(x: f32, y: f32, color: Color, mesh: Mesh, hp_modifier: Option<f32>) -> Actor {
    let hp = 5.0 * hp_modifier.unwrap_or(1.0);
    // log(n) max velocity of 550.0
    let velocity = 600.0_f32.ln() * hp_modifier.unwrap_or(1.0) + 100.0;
    Actor {
        actor_type: ActorType::Enemy,
        x,
        y,
        target_x: x,
        target_y: y,
        velocity,
        color,
        hp,
        bounding_box: mesh,
        is_taking_damage: None,
    }
}
pub fn create_boss_enemy(
    x: f32,
    y: f32,
    color: Color,
    hp_scale_factor: f32,
    velocity_scale_factor: f32,
    mesh: Mesh,
) -> Actor {
    Actor {
        actor_type: ActorType::Enemy,
        x,
        y,
        target_x: x,
        target_y: y,
        velocity: 100.0 * velocity_scale_factor,
        color,
        hp: 1000.0 * hp_scale_factor,
        bounding_box: mesh,
        is_taking_damage: None,
    }
}
