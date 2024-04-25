use crate::actors::models::{Actor, ActorType};
use ggez::graphics;
use ggez::graphics::Mesh;

pub fn handle_timed_life(actor: &mut Actor, dt: f32) {
    actor.hp -= dt;
}

pub fn create_player_projectile(
    x: f32,
    y: f32,
    target_x: f32,
    target_y: f32,
    mesh: Mesh,
    damage_modifier: Option<f32>,
) -> Actor {
    let damage = 10.0 * damage_modifier.unwrap_or(1.0);
    Actor {
        actor_type: ActorType::PlayerProjectile,
        x,
        y,
        target_x,
        target_y,
        velocity: 800.0,
        color: graphics::Color::new(0.8, 0.8, 1.0, 1.0),
        hp: damage,
        bounding_box: mesh,
        is_taking_damage: None,
        attack_cooldown: None,
        ai: None,
    }
}

pub fn create_player_alt_projectile(
    x: f32,
    y: f32,
    target_x: f32,
    target_y: f32,
    mesh: Mesh,
    damage_modifier: Option<f32>,
) -> Actor {
    let damage = 10.0 * damage_modifier.unwrap_or(1.0);
    Actor {
        actor_type: ActorType::PlayerProjectile,
        x,
        y,
        target_x,
        target_y,
        velocity: 1000.0,
        color: graphics::Color::new(0.4, 0.8, 0.9, 0.75),
        hp: damage,
        bounding_box: mesh,
        is_taking_damage: None,
        attack_cooldown: Some(5000.0),
        ai: None,
    }
}

pub fn create_enemy_projectile(
    x: f32,
    y: f32,
    target_x: f32,
    target_y: f32,
    mesh: Mesh,
    damage_modifier: Option<f32>,
) -> Actor {
    let damage = 10.0 * damage_modifier.unwrap_or(1.0);
    Actor {
        actor_type: ActorType::EnemyProjectile,
        x,
        y,
        target_x,
        target_y,
        velocity: 800.0,
        color: graphics::Color::new(1.0, 0.3, 0.3, 0.8),
        hp: damage,
        bounding_box: mesh,
        is_taking_damage: None,
        attack_cooldown: None,
        ai: None,
    }
}

pub fn create_boss_enemy_projectile(
    x: f32,
    y: f32,
    target_x: f32,
    target_y: f32,
    mesh: Mesh,
    damage_modifier: Option<f32>,
) -> Actor {
    let damage = 5.0 * damage_modifier.unwrap_or(1.0);
    Actor {
        actor_type: ActorType::EnemyProjectile,
        x,
        y,
        target_x,
        target_y,
        velocity: 800.0,
        color: graphics::Color::new(1.0, 0.3, 0.3, 0.8),
        hp: damage,
        bounding_box: mesh,
        is_taking_damage: None,
        attack_cooldown: None,
        ai: None,
    }
}
