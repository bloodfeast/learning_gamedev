use crate::actors::models::{Actor, ActorType};
use ggez::graphics::{Color, Image, Mesh};

pub fn create_player(x: f32, y: f32, color: Color, mesh: Mesh, image: Option<Image>) -> Actor {
    Actor {
        actor_type: ActorType::Player,
        x,
        y,
        target_x: x,
        target_y: y,
        velocity: 0.1,
        color,
        hp: 100.0,
        bounding_box: mesh,
        image,
        is_taking_damage: None,
        attack_cooldown: None,
        ai: None,
    }
}
