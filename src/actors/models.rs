use ggez::graphics;
use ggez::graphics::Color;
use ggez::mint::Point2;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorType {
    Player,
    Enemy,
    BossEnemy,
    PlayerProjectile,
    EnemyProjectile,
}

#[derive(Debug, Clone)]
pub struct Actor {
    pub actor_type: ActorType,
    pub x: f32,
    pub y: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub velocity: f32,
    pub color: graphics::Color,
    pub hp: f32,
    pub bounding_box: graphics::Mesh,
    pub is_taking_damage: Option<f32>,
    pub attack_cooldown: Option<f32>,
}

pub fn get_player_polygon_mesh_vertices() -> Vec<Point2<f32>> {
    vec![
        Point2 { x: 0.0, y: -25.0 },
        Point2 { x: 2.0, y: -25.0 },
        Point2 { x: 6.0, y: -26.0 },
        Point2 { x: 10.0, y: -16.0 },
        Point2 { x: 16.0, y: -8.0 },
        Point2 { x: 4.0, y: -12.0 },
        Point2 { x: 4.0, y: -10.0 },
        // middle
        Point2 { x: -4.0, y: -10.0 },
        Point2 { x: -4.0, y: -12.0 },
        Point2 { x: -16.0, y: -8.0 },
        Point2 { x: -10.0, y: -16.0 },
        Point2 { x: -4.0, y: -26.0 },
        Point2 { x: -2.0, y: -25.0 },
        Point2 { x: 0.0, y: -25.0 },
    ]
}

pub fn get_enemy_polygon_mesh_vertices() -> Vec<Point2<f32>> {
    vec![
        Point2 { x: 0.0, y: 25.0 },
        Point2 { x: 2.0, y: 25.0 },
        Point2 { x: 6.0, y: 26.0 },
        Point2 { x: 10.0, y: 16.0 },
        Point2 { x: 16.0, y: 8.0 },
        Point2 { x: 4.0, y: 12.0 },
        Point2 { x: 4.0, y: 10.0 },
        // middle
        Point2 { x: -4.0, y: 10.0 },
        Point2 { x: -4.0, y: 12.0 },
        Point2 { x: -16.0, y: 8.0 },
        Point2 { x: -10.0, y: 16.0 },
        Point2 { x: -4.0, y: 26.0 },
        Point2 { x: -2.0, y: 25.0 },
        Point2 { x: 0.0, y: 25.0 },
    ]
}
pub fn get_boss_enemy_polygon_mesh_vertices() -> Vec<Point2<f32>> {
    vec![
        Point2 { x: 0.0, y: 55.0 },
        Point2 { x: 32.0, y: 55.0 },
        Point2 { x: 46.0, y: 56.0 },
        Point2 { x: 50.0, y: 46.0 },
        Point2 { x: 56.0, y: 38.0 },
        Point2 { x: 44.0, y: 42.0 },
        Point2 { x: 44.0, y: 40.0 },
        // middle
        Point2 { x: -44.0, y: 40.0 },
        Point2 { x: -44.0, y: 42.0 },
        Point2 { x: -56.0, y: 38.0 },
        Point2 { x: -50.0, y: 46.0 },
        Point2 { x: -46.0, y: 56.0 },
        Point2 { x: -32.0, y: 55.0 },
        Point2 { x: 0.0, y: 55.0 },
    ]
}

pub fn get_projectile_mesh_vertices() -> Vec<Point2<f32>> {
    vec![
        Point2 { x: 0.0, y: 0.0 },
        Point2 { x: 5.0, y: 2.5 },
        Point2 { x: 5.0, y: 5.0 },
        Point2 { x: 0.0, y: 2.5 },
        Point2 { x: 0.0, y: 0.0 },
    ]
}
pub fn get_player_alt_projectile_mesh_vertices() -> Vec<Point2<f32>> {
    vec![
        Point2 { x: -20.0, y: -10.0 },
        Point2 { x: -15.0, y: -20.0 },
        Point2 { x: 0.0, y: -30.0 },
        Point2 { x: 15.0, y: -20.0 },
        Point2 { x: 20.0, y: -10.0 },
        Point2 { x: 10.0, y: 0.0 },
        Point2 { x: 0.0, y: 0.0 },
        Point2 { x: -10.0, y: 0.0 },
        Point2 { x: -20.0, y: -10.0 },
    ]
}

pub fn take_damage(actor: &mut Actor, damage: &f32) -> () {
    actor.hp -= damage;
}

pub fn create_spaceship_mesh(ctx: &mut ggez::Context) -> graphics::Mesh {
    let player_mesh = graphics::Mesh::new_polygon(
        ctx,
        graphics::DrawMode::fill(),
        get_player_polygon_mesh_vertices().deref(),
        Color::from_rgb(150, 200, 150),
    )
    .expect("Failed to create spaceship mesh");
    player_mesh
}

pub fn create_enemy_spaceship_mesh(ctx: &mut ggez::Context) -> graphics::Mesh {
    let player_mesh = graphics::Mesh::new_polygon(
        ctx,
        graphics::DrawMode::fill(),
        get_enemy_polygon_mesh_vertices().deref(),
        Color::from_rgb(200, 100, 100),
    )
    .expect("Failed to create enemy spaceship mesh");
    player_mesh
}

pub fn create_boss_enemy_spaceship_mesh(ctx: &mut ggez::Context) -> graphics::Mesh {
    let player_mesh = graphics::Mesh::new_polygon(
        ctx,
        graphics::DrawMode::fill(),
        get_boss_enemy_polygon_mesh_vertices().deref(),
        Color::from_rgb(200, 50, 30),
    )
    .expect("Failed to create enemy spaceship mesh");
    player_mesh
}

pub fn create_player_projectile_mesh(ctx: &mut ggez::Context) -> graphics::Mesh {
    let projectile_mesh = graphics::Mesh::new_polygon(
        ctx,
        graphics::DrawMode::fill(),
        get_projectile_mesh_vertices().deref(),
        Color::from_rgb(50, 200, 250),
    )
    .expect("Failed to create player projectile mesh");
    projectile_mesh
}

pub fn create_enemy_projectile_mesh(ctx: &mut ggez::Context) -> graphics::Mesh {
    let projectile_mesh = graphics::Mesh::new_polygon(
        ctx,
        graphics::DrawMode::fill(),
        get_projectile_mesh_vertices().deref(),
        Color::from_rgb(250, 100, 150),
    )
    .expect("Failed to create enemy projectile mesh");
    projectile_mesh
}
pub fn create_boss_enemy_projectile_mesh(ctx: &mut ggez::Context) -> graphics::Mesh {
    let projectile_mesh = graphics::Mesh::new_polygon(
        ctx,
        graphics::DrawMode::fill(),
        get_player_alt_projectile_mesh_vertices().deref(),
        Color::from_rgb(250, 100, 150),
    )
    .expect("Failed to create enemy projectile mesh");
    projectile_mesh
}

pub fn create_player_alt_projectile_mesh(ctx: &mut ggez::Context) -> graphics::Mesh {
    let projectile_mesh = graphics::Mesh::new_polygon(
        ctx,
        graphics::DrawMode::fill(),
        get_player_alt_projectile_mesh_vertices().deref(),
        Color::from_rgba(50, 210, 220, 200),
    )
    .expect("Failed to create player alt projectile mesh");
    projectile_mesh
}
