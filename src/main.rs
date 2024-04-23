mod actors;
mod asset_manager;

use crate::actors::enemy::{create_boss_enemy, create_enemy};
use crate::actors::models::{
    create_boss_enemy_projectile_mesh, create_boss_enemy_spaceship_mesh,
    create_enemy_projectile_mesh, create_enemy_spaceship_mesh, create_player_alt_projectile_mesh,
    create_player_projectile_mesh, create_spaceship_mesh, take_damage, Actor, ActorType,
};
use crate::actors::player::create_player;
use crate::actors::projectile::{
    create_boss_enemy_projectile, create_enemy_projectile, create_player_alt_projectile,
    create_player_projectile, handle_timed_life,
};
use crate::asset_manager::Assets;
use ggez::audio::SoundSource;
use ggez::conf::NumSamples;
use ggez::event::MouseButton;
use ggez::graphics::{Canvas, Color, Drawable, Text};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::input::mouse::CursorIcon;
use ggez::mint::Point2;
use ggez::winit::dpi::LogicalPosition;
use ggez::winit::window::{CursorGrabMode, WindowLevel};
use ggez::*;
use rand::prelude::*;
use std::collections::HashSet;
use std::time::Duration;
use std::{env, path};

struct GameState {
    dt: Duration,
    assets: Assets,
    player: Actor,
    enemy: Vec<Actor>,
    projectiles: Vec<Actor>,
    keys_pressed: HashSet<KeyCode>,
    kills: u64,
    alt_cd: f32,
    game_state_data: std::collections::HashMap<String, f32>,
}
fn handle_player_movement(
    player: &mut Actor,
    keys_pressed: &HashSet<KeyCode>,
    dt: Duration,
    screen_width: f32,
    screen_height: f32,
) {
    let mut is_moving = false;
    if keys_pressed.contains(&KeyCode::W) {
        player.target_y -= player.velocity * dt.as_secs_f32();
        is_moving = true;
    }
    if keys_pressed.contains(&KeyCode::S) {
        player.target_y += player.velocity * dt.as_secs_f32();
        is_moving = true;
    }
    if keys_pressed.contains(&KeyCode::A) {
        player.target_x -= player.velocity * dt.as_secs_f32();
        is_moving = true;
    }
    if keys_pressed.contains(&KeyCode::D) {
        player.target_x += player.velocity * dt.as_secs_f32();
        is_moving = true;
    }
    // Check if the player is outside the screen boundaries
    if player.x <= 0.0 {
        player.x = 0.0;
        player.target_x = player.x + 10.0;
    } else if player.x >= screen_width {
        player.x = screen_width;
        player.target_x = player.x - 10.0;
    }

    if player.y <= 0.0 {
        player.y = 0.0;
        player.target_y = player.y + 10.0;
    } else if player.y >= screen_height {
        player.y = screen_height;
        player.target_y = player.y - 10.0;
    }
    // If no movement keys are pressed, reduce the actor's velocity to simulate deceleration
    if !is_moving && player.velocity > 0.0 {
        player.velocity -= 1.0 * dt.as_millis() as f32; // Deceleration factor of 10.0
        if player.velocity < 1.0 {
            player.velocity = 0.0;
        }
    } else if is_moving {
        player.velocity += 1.0 * dt.as_millis() as f32; // Deceleration factor of 10.0
        if player.velocity > 800.0 {
            player.velocity = 800.0;
        }
    }
    // Calculate the direction vector from the actor's current position to the target position
    let direction = ((player.target_x - player.x), (player.target_y - player.y));

    // Calculate the length of the direction vector
    let length = (direction.0.powi(2) + direction.1.powi(2)).sqrt();

    // Normalize the direction vector to get a unit direction vector
    let unit_direction = if length > 0.0 {
        (direction.0 / length, direction.1 / length)
    } else {
        (0.0, 0.0)
    };
    // Update the actor's velocity
    // Calculate the movement vector by multiplying the unit direction vector by the actor's velocity and the elapsed time
    let movement = (
        unit_direction.0 * player.velocity * dt.as_secs_f32(),
        unit_direction.1 * player.velocity * dt.as_secs_f32(),
    );
    // Update the actor's position
    player.x += movement.0;
    player.y += movement.1;
}

fn handle_enemy_movement(enemy: &mut Actor, dt: Duration) {
    // Calculate the direction vector from the actor's current position to the target position
    let direction = ((enemy.target_x - enemy.x), (enemy.target_y - enemy.y));

    // Calculate the length of the direction vector
    let length = (direction.0.powi(2) + direction.1.powi(2)).sqrt();

    enemy.attack_cooldown = match enemy.attack_cooldown {
        Some(mut cd) => {
            if cd <= 0.0 {
                cd = 0.0;
            } else {
                cd -= dt.as_millis() as f32;
            }
            Some(cd)
        }
        None => None,
    };

    // Normalize the direction vector to get a unit direction vector
    let unit_direction = if length > 0.0 {
        (direction.0 / length, direction.1 / length)
    } else {
        (0.0, 0.0)
    };
    // Update the actor's velocity
    // Calculate the movement vector by multiplying the unit direction vector by the actor's velocity and the elapsed time
    let movement = (
        unit_direction.0 * enemy.velocity * dt.as_secs_f32(),
        unit_direction.1 * enemy.velocity * dt.as_secs_f32(),
    );
    // Update the actor's position
    enemy.x += movement.0;
    enemy.y += movement.1;
}

fn handle_projectile_trajectory(projectile: &mut Actor, dt: Duration) {
    // Calculate the direction vector from the actor's current position to the target position
    let direction = (
        (projectile.target_x - projectile.x),
        (projectile.target_y - projectile.y),
    );

    // Calculate the length of the direction vector
    let length = (direction.0.powi(2) + direction.1.powi(2)).sqrt();

    // Normalize the direction vector to get a unit direction vector
    let unit_direction = if length > 0.0 {
        (direction.0 / length, direction.1 / length)
    } else {
        (0.0, 0.0)
    };
    // Update the actor's velocity
    // Calculate the movement vector by multiplying the unit direction vector by the actor's velocity and the elapsed time
    let movement = (
        unit_direction.0 * projectile.velocity * dt.as_secs_f32(),
        unit_direction.1 * projectile.velocity * dt.as_secs_f32(),
    );
    // Update the actor's position
    projectile.x += movement.0;
    projectile.y += movement.1;
}

impl event::EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        let screen_width = ctx.gfx.window().inner_size().width as f32;
        let screen_height = ctx.gfx.window().inner_size().height as f32;
        if self.alt_cd <= 0.0 {
            self.alt_cd = 0.0;
        } else {
            self.alt_cd -= self.dt.as_millis() as f32;
        }
        handle_player_movement(
            &mut self.player,
            &self.keys_pressed,
            self.dt,
            screen_width,
            screen_height,
        );
        let player_coords = (self.player.x, self.player.y);

        for i in 0..self.enemy.len() {
            let (x, y) = match ctx.time.time_since_start().as_millis() {
                t if t >= 60000 && t % 1000 == 0 => (player_coords.0, player_coords.1),
                t if t % 600 == 0 && t < 60000 => {
                    // rng to determine the target position of the enemy
                    let mut rng = rand::thread_rng();
                    let x = rng.gen_range(player_coords.0..(player_coords.0 + 1000.0));
                    let y = player_coords.1;
                    (x, y)
                }
                t if t % 500 == 0 && t < 60000 => {
                    // rng to determine the target position of the enemy
                    let mut rng = rand::thread_rng();
                    let x = rng.gen_range((player_coords.0 - 1000.0)..player_coords.0);
                    let y = player_coords.1;
                    (x, y)
                }
                _ => (self.enemy[i].target_x, self.enemy[i].target_y),
            };
            self.enemy[i].target_x = x;
            self.enemy[i].target_y = y;

            // Prevent enemies from colliding with each other
            for j in 0..self.enemy.len() {
                if i != j {
                    let other_enemy = &self.enemy[j];
                    let distance = ((self.enemy[i].x - other_enemy.x).powi(2)
                        + (self.enemy[i].y - other_enemy.y).powi(2))
                    .sqrt();
                    if distance
                        < self.enemy[i]
                            .bounding_box
                            .dimensions(ctx)
                            .expect("Failed to get bounding box dimensions")
                            .w
                            + 20.0
                    {
                        // Calculate the direction vector from other_enemy to enemy
                        let direction = (
                            self.enemy[i].x - other_enemy.x,
                            self.enemy[i].y - other_enemy.y,
                        );

                        // Normalize the direction vector
                        let length = (direction.0.powi(2) + direction.1.powi(2)).sqrt();
                        let unit_direction = if length > 0.0 {
                            (direction.0 / length, direction.1 / length)
                        } else {
                            (0.0, 0.0)
                        };

                        // Set the target position to a point in the opposite direction
                        self.enemy[i].target_x = self.enemy[i].x + unit_direction.0 * 10.0;
                        self.enemy[i].target_y = self.enemy[i].y + unit_direction.1 * 10.0;
                    }
                }
            }

            handle_enemy_movement(&mut self.enemy[i], self.dt);

            if self.enemy[i].attack_cooldown == Some(0.0) {
                let aim_x = player_coords.0 + rand::thread_rng().gen_range(-420.0..420.0);
                let aim_y = player_coords.1 + rand::thread_rng().gen_range(-420.0..420.0);
                let direction = ((aim_x - self.enemy[i].x), (aim_y - self.enemy[i].y));
                // Calculate the length of the direction vector
                let length = (direction.0.powi(2) + direction.1.powi(2)).sqrt();
                // Normalize the direction vector to get a unit direction vector
                let unit_direction = match length > 0.0 {
                    true => (direction.0 / length, direction.1 / length),
                    false => (0.0, 0.0),
                };
                if self.enemy[i].actor_type == ActorType::BossEnemy {
                    // Multiply the unit direction vector by a large number to get a far away target position
                    let far_away_target = (
                        player_coords.0 + unit_direction.0 * 1000.0,
                        player_coords.1 + unit_direction.1 * 1000.0,
                    );
                    let boss_kills = self.game_state_data.get("boss_count");
                    let boss_kills = match boss_kills {
                        Some(count) => count * 5.0,
                        None => 5.0,
                    };
                    for _ in 0..=boss_kills as u32 {
                        let projectile_type = rand::thread_rng().gen_range(0..2);
                        match projectile_type {
                            0 => {
                                let projectile = create_enemy_projectile(
                                    self.enemy[i].x,
                                    self.enemy[i].y,
                                    far_away_target.0,
                                    far_away_target.1,
                                    create_enemy_projectile_mesh(ctx),
                                    Some(1.0),
                                );
                                self.assets.laser_1.set_volume(0.4);
                                let res = self.assets.laser_1.play(ctx);
                                match res {
                                    Ok(_) => (),
                                    Err(e) => println!("Error playing laser_1: {:?}", e),
                                }
                                self.projectiles.push(projectile);
                            }
                            1 => {
                                let projectile = create_boss_enemy_projectile(
                                    self.enemy[i].x,
                                    self.enemy[i].y,
                                    far_away_target.0,
                                    far_away_target.1,
                                    create_boss_enemy_projectile_mesh(ctx),
                                    Some(1.0),
                                );
                                self.assets.laser_1.set_volume(0.4);
                                let res = self.assets.laser_1.play(ctx);
                                match res {
                                    Ok(_) => (),
                                    Err(e) => println!("Error playing laser_1: {:?}", e),
                                }
                                self.projectiles.push(projectile);
                            }
                            _ => (),
                        }
                    }
                    self.enemy[i].attack_cooldown = Some(thread_rng().gen_range(50.0..250.0));
                } else {
                    // Multiply the unit direction vector by a large number to get a far away target position
                    let far_away_target = (
                        player_coords.0 + unit_direction.0 * 10000.0,
                        player_coords.1 + unit_direction.1 * 10000.0,
                    );
                    let projectile = create_enemy_projectile(
                        self.enemy[i].x,
                        self.enemy[i].y,
                        far_away_target.0,
                        far_away_target.1,
                        create_enemy_projectile_mesh(ctx),
                        Some(1.0),
                    );

                    self.assets.laser_1.set_volume(0.4);
                    let res = self.assets.laser_1.play(ctx);
                    match res {
                        Ok(_) => (),
                        Err(e) => println!("Error playing laser_1: {:?}", e),
                    }
                    self.enemy[i].attack_cooldown = Some(thread_rng().gen_range(500.0..5000.0));
                    self.projectiles.push(projectile);
                }
            }
        }
        for projectile in &mut self.projectiles {
            // Check if the projectile is outside the screen boundaries
            handle_timed_life(projectile, self.dt.as_secs_f32());
            handle_projectile_trajectory(projectile, self.dt);
            // Check if the projectile is outside the screen boundaries
            if projectile.x <= 0.0
                || projectile.x >= screen_width
                || projectile.y <= 0.0
                || projectile.y >= screen_height
            {
                projectile.hp = 0.0;
            }
        }

        // Check for collisions between the player and the enemy
        for enemy in &mut self.enemy {
            let distance =
                ((enemy.x - self.player.x).powi(2) + (enemy.y - self.player.y).powi(2)).sqrt();
            if distance
                < self
                    .player
                    .bounding_box
                    .dimensions(ctx)
                    .expect("Failed to get bounding box dimensions")
                    .w
            {
                let player_hp = self.player.hp.clone();
                let enemy_hp = &enemy.hp.clone();
                take_damage(&mut self.player, enemy_hp);
                take_damage(enemy, &player_hp);

                self.assets.damage.set_volume(0.5);
                let res = self.assets.damage.play(ctx);
                match res {
                    Ok(_) => (),
                    Err(e) => println!("Error playing bgm: {:?}", e),
                }
            }
        }

        // Check for collisions between the player and the projectiles
        for projectile in &mut self.projectiles {
            let distance = ((projectile.x - self.player.x).powi(2)
                + (projectile.y - self.player.y).powi(2))
            .sqrt();
            if distance
                < self
                    .player
                    .bounding_box
                    .dimensions(ctx)
                    .expect("Failed to get bounding box dimensions")
                    .w
            {
                if projectile.actor_type == ActorType::EnemyProjectile {
                    let hp = self.player.hp;
                    take_damage(&mut self.player, &projectile.hp);

                    self.assets.damage.set_volume(0.5);
                    let res = self.assets.damage.play(ctx);
                    match res {
                        Ok(_) => (),
                        Err(e) => println!("Error playing bgm: {:?}", e),
                    }
                    projectile.hp -= hp;
                    if projectile.hp <= 0.0 {
                        projectile.hp = 0.0;
                    }
                }
            }
        }

        // Check for collisions between the enemy and the projectiles
        for enemy in &mut self.enemy {
            for projectile in &mut self.projectiles {
                if projectile.actor_type == ActorType::EnemyProjectile {
                    continue;
                }
                let distance =
                    ((projectile.x - enemy.x).powi(2) + (projectile.y - enemy.y).powi(2)).sqrt();
                if distance
                    < enemy
                        .bounding_box
                        .dimensions(ctx)
                        .expect("Failed to get bounding box dimensions")
                        .w
                {
                    let hp = enemy.hp.clone();
                    take_damage(enemy, &projectile.hp);
                    projectile.hp -= hp;
                    if projectile.hp <= 0.0 {
                        projectile.hp = 0.0;
                    }
                }
            }
            if enemy.hp <= 0.0 {
                if enemy.actor_type == ActorType::BossEnemy {
                    self.kills += 10;
                } else {
                    self.kills += 1;
                }
            }
        }

        self.projectiles.retain(|projectile| projectile.hp > 0.0);
        self.enemy.retain(|enemy| enemy.hp > 0.0);
        if self.enemy.is_empty() {
            let wave_count = self.game_state_data.get("wave_count");
            let wave_count = match wave_count {
                Some(count) => count + &1.0,
                None => 1.0,
            };
            self.game_state_data
                .insert("wave_count".to_string(), wave_count);
            let mut is_boss_round = false;
            let boss_count = self.game_state_data.get("boss_count");
            let boss_count = match boss_count {
                Some(count) => count + &1.0,
                None => 1.0,
            };
            let is_eligible_for_boss = wave_count % 5.0 == 0.0;
            if is_eligible_for_boss {
                is_boss_round = true;
                self.enemy.push(create_boss_enemy(
                    900.0,
                    500.0,
                    Color::RED,
                    wave_count,
                    wave_count,
                    create_boss_enemy_spaceship_mesh(ctx),
                    Some(250_f32),
                ));
                self.game_state_data
                    .insert("boss_count".to_string(), boss_count);
            }
            if !is_boss_round {
                for _ in 0..(wave_count * 1.75).ceil() as u32 {
                    let mut rng = rand::thread_rng();
                    let mut x_nums: Vec<i32> = (0..1800).collect();
                    let mut y_nums: Vec<i32> = (100..900).collect();
                    x_nums.shuffle(&mut rng);
                    y_nums.shuffle(&mut rng);

                    x_nums.retain(|&x| {
                        x < (player_coords.0 - 400.0) as i32 || x > (player_coords.0 + 400.0) as i32
                    });
                    y_nums.retain(|&y| {
                        y < (player_coords.1 - 400.0) as i32 || y > (player_coords.1 + 400.0) as i32
                    });
                    let attack_cd = if self.kills > 1 {
                        Some(rng.gen_range(1000.0..5000.0))
                    } else {
                        None
                    };

                    let enemy = create_enemy(
                        x_nums[0] as f32,
                        y_nums[0] as f32,
                        Color::RED,
                        create_enemy_spaceship_mesh(ctx),
                        Some(self.kills as f32 * 1.10),
                        attack_cd,
                    );
                    self.enemy.push(enemy);
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        let fps = Text::new(format!("FPS: {:.2}", ctx.time.fps()));
        let mut kill_count = Text::new(format!("Kills: {}", self.kills));

        let background_pos = *self
            .game_state_data
            .get("background_tile_1_y_pos")
            .get_or_insert(&0.0);
        let binding = &0.0 + &(self.assets.background.height() as f32);
        let background_pos_2 = *self
            .game_state_data
            .get("background_tile_2_y_pos")
            .get_or_insert(&binding);

        fn get_background_pos(background_pos: &f32, background_height: f32) -> f32 {
            if background_pos >= &background_height {
                20.0 - &background_height
            } else {
                background_pos + 20.0
            }
        }

        let background_pos =
            get_background_pos(&background_pos, self.assets.background.height() as f32);
        let background_pos_2 =
            get_background_pos(&background_pos_2, self.assets.background.height() as f32);

        self.game_state_data
            .insert("background_tile_1_y_pos".to_string(), background_pos);
        self.game_state_data
            .insert("background_tile_2_y_pos".to_string(), background_pos_2);

        canvas.draw(&self.assets.background, Point2::from([0.0, background_pos]));
        canvas.draw(
            &self.assets.background,
            Point2::from([0.0, background_pos_2]),
        );

        fps.draw(&mut canvas, Point2::from([10.0, 10.0]));
        kill_count.draw(&mut canvas, Point2::from([1500.0, 30.0]));
        let mut damage_modifier = ((self.kills / 10) as f32).floor() * 1.25;
        if damage_modifier == 0.0 {
            damage_modifier = 1.0;
        }
        let player_damage = Text::new(format!("Atk Dmg: {}", 10.0 * damage_modifier));
        player_damage.draw(&mut canvas, Point2::from([1700.0, 30.0]));
        let mut alt_damage_modifier = ((self.kills / 30) as f32).floor() * 100.0;
        if alt_damage_modifier == 0.0 {
            alt_damage_modifier = 100.0;
        }
        let alt_damage = Text::new(format!("Power Atk Dmg: {}", 10.0 * alt_damage_modifier));
        alt_damage.draw(&mut canvas, Point2::from([1700.0, 50.0]));
        let alt_cd = Text::new(format!("Power Atk CD: {:.2}ms", self.alt_cd));
        alt_cd.draw(&mut canvas, Point2::from([1700.0, 70.0]));

        if self.player.hp <= 0.0 {
            let mut game_over_text = Text::new("Game Over");
            game_over_text.set_scale(50.0);
            kill_count.set_scale(30.0);
            game_over_text.draw(&mut canvas, Point2::from([800.0, 500.0]));
            kill_count.draw(&mut canvas, Point2::from([900.0, 580.0]));
            self.enemy.drain(..);
            self.projectiles.drain(..);
            return canvas.finish(ctx);
        }
        self.player
            .bounding_box
            .draw(&mut canvas, Point2::from([self.player.x, self.player.y]));

        let _ = &self.enemy.iter().for_each(|enemy| {
            enemy
                .bounding_box
                .draw(&mut canvas, Point2::from([enemy.x, enemy.y]));
        });

        let _ = &self.projectiles.iter().for_each(|projectile| {
            projectile
                .bounding_box
                .draw(&mut canvas, Point2::from([projectile.x, projectile.y]));
        });
        if !self.assets.bgm.playing() {
            self.assets.bgm.set_volume(0.45);
            self.assets.bgm.set_fade_in(Duration::from_millis(5000));
            let res = self.assets.bgm.play(ctx);
            match res {
                Ok(_) => (),
                Err(e) => println!("Error playing bgm: {:?}", e),
            }
        }
        if self.kills > 30 && self.kills < 50 {
            let alert_text = Text::new(format!("Special Attack Unlocked! (RMB)"));
            alert_text.draw(&mut canvas, Point2::from([800.0, 30.0]));
        }

        canvas.finish(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), GameError> {
        // Create a player projectile when the space key is pressed
        let player = &self.player;
        if player.hp <= 0.0 {
            return Ok(());
        }
        // Calculate the direction vector from the player's current position to the mouse position
        let direction = ((x - player.x), (y - player.y));
        // Calculate the length of the direction vector
        let length = (direction.0.powi(2) + direction.1.powi(2)).sqrt();
        // Normalize the direction vector to get a unit direction vector
        let unit_direction = match length > 0.0 {
            true => (direction.0 / length, direction.1 / length),
            false => (0.0, 0.0),
        };
        // Multiply the unit direction vector by a large number to get a far away target position
        let far_away_target = (
            player.x + unit_direction.0 * 10000.0,
            player.y + unit_direction.1 * 10000.0,
        );
        // Get offset from player to projectile spawn point
        let projectile_spawn_y_offset = if unit_direction.1 > 0.0 { 2.0 } else { -24.0 };
        let projectile_spawn_x_offset = if unit_direction.0 > 0.0 { 10.0 } else { -10.0 };
        let projectiles: Option<Vec<Actor>> = match button {
            MouseButton::Left => {
                let mut modifier = Some(((self.kills / 10) as f32).floor() * 1.5);
                if modifier.expect(
                    "Failed to get damage modifier for player projectile. This should never happen",
                ) == 0.0
                {
                    modifier = None;
                }
                match self.kills {
                    kills if kills > 40 => {
                        let mut split_projectiles = vec![];
                        for i in 0..5 {
                            match i {
                                0 => {
                                    let projectile = create_player_projectile(
                                        player.x + projectile_spawn_x_offset,
                                        player.y + projectile_spawn_y_offset,
                                        far_away_target.0 - 800.0,
                                        far_away_target.1 - 400.0,
                                        create_player_projectile_mesh(ctx),
                                        modifier,
                                    );
                                    split_projectiles.push(projectile);
                                }
                                1 => {
                                    let projectile = create_player_projectile(
                                        player.x + projectile_spawn_x_offset,
                                        player.y + projectile_spawn_y_offset,
                                        far_away_target.0 - 400.0,
                                        far_away_target.1 - 200.0,
                                        create_player_projectile_mesh(ctx),
                                        modifier,
                                    );
                                    split_projectiles.push(projectile);
                                }
                                2 => {
                                    let projectile = create_player_projectile(
                                        player.x + projectile_spawn_x_offset,
                                        player.y + projectile_spawn_y_offset,
                                        far_away_target.0,
                                        far_away_target.1,
                                        create_player_projectile_mesh(ctx),
                                        modifier,
                                    );
                                    split_projectiles.push(projectile);
                                }
                                3 => {
                                    let projectile = create_player_projectile(
                                        player.x + projectile_spawn_x_offset,
                                        player.y + projectile_spawn_y_offset,
                                        far_away_target.0 + 400.0,
                                        far_away_target.1 + 200.0,
                                        create_player_projectile_mesh(ctx),
                                        modifier,
                                    );
                                    split_projectiles.push(projectile);
                                }
                                4 => {
                                    let projectile = create_player_projectile(
                                        player.x + projectile_spawn_x_offset,
                                        player.y + projectile_spawn_y_offset,
                                        far_away_target.0 + 800.0,
                                        far_away_target.1 + 400.0,
                                        create_player_projectile_mesh(ctx),
                                        modifier,
                                    );
                                    split_projectiles.push(projectile);
                                }
                                _ => (),
                            }
                        }

                        self.assets.spread_shot_5.set_volume(0.425);
                        let res = self.assets.spread_shot_5.play(ctx);
                        match res {
                            Ok(_) => (),
                            Err(e) => println!("Error playing bgm: {:?}", e),
                        }
                        Some(split_projectiles)
                    }
                    kills if kills > 20 => {
                        let mut split_projectiles = vec![];
                        for i in 0..3 {
                            match i {
                                0 => {
                                    let projectile = create_player_projectile(
                                        player.x + projectile_spawn_x_offset,
                                        player.y + projectile_spawn_y_offset,
                                        far_away_target.0 - 500.0,
                                        far_away_target.1 - 200.0,
                                        create_player_projectile_mesh(ctx),
                                        modifier,
                                    );
                                    split_projectiles.push(projectile);
                                }
                                1 => {
                                    let projectile = create_player_projectile(
                                        player.x + projectile_spawn_x_offset,
                                        player.y + projectile_spawn_y_offset,
                                        far_away_target.0,
                                        far_away_target.1,
                                        create_player_projectile_mesh(ctx),
                                        modifier,
                                    );
                                    split_projectiles.push(projectile);
                                }
                                2 => {
                                    let projectile = create_player_projectile(
                                        player.x + projectile_spawn_x_offset,
                                        player.y + projectile_spawn_y_offset,
                                        far_away_target.0 + 500.0,
                                        far_away_target.1 + 200.0,
                                        create_player_projectile_mesh(ctx),
                                        modifier,
                                    );
                                    split_projectiles.push(projectile);
                                }
                                _ => (),
                            }
                        }

                        self.assets.spread_shot_3.set_volume(0.425);
                        let res = self.assets.spread_shot_3.play(ctx);
                        match res {
                            Ok(_) => (),
                            Err(e) => println!("Error playing bgm: {:?}", e),
                        }
                        Some(split_projectiles)
                    }
                    _ => {
                        // Create a player projectile at the player's position
                        let projectile = create_player_projectile(
                            player.x + projectile_spawn_x_offset,
                            player.y + projectile_spawn_y_offset,
                            far_away_target.0,
                            far_away_target.1,
                            create_player_projectile_mesh(ctx),
                            modifier,
                        );

                        self.assets.player_laser_1.set_volume(0.4);
                        let res = self.assets.player_laser_1.play(ctx);
                        match res {
                            Ok(_) => (),
                            Err(e) => println!("Error playing bgm: {:?}", e),
                        }
                        Some(vec![projectile])
                    }
                }
            }
            MouseButton::Right => {
                let alt_projectile: Option<Actor> = match self.alt_cd {
                    cd if cd <= 0.0 => {
                        if self.kills < 30 {
                            None
                        } else {
                            self.alt_cd = 5000.0;
                            let projectile_spawn_y_offset =
                                if unit_direction.1 > 0.0 { 20.0 } else { -20.0 };
                            let projectile_spawn_x_offset =
                                if unit_direction.0 > 0.0 { 20.0 } else { -20.0 };
                            let mut modifier = Some(((self.kills / 30) as f32).floor() * 100.0);
                            if modifier
                                .expect(
                                    "Failed to get modifier. This should never happen as the modifier is checked for None",
                                ) == 0.0 {
                                modifier = Some(100.0);
                            }

                            self.assets.special_atk.set_volume(0.43);
                            let res = self.assets.special_atk.play(ctx);
                            match res {
                                Ok(_) => (),
                                Err(e) => println!("Error playing bgm: {:?}", e),
                            }
                            Some(create_player_alt_projectile(
                                player.x + projectile_spawn_x_offset,
                                player.y + projectile_spawn_y_offset,
                                far_away_target.0,
                                far_away_target.1,
                                create_player_alt_projectile_mesh(ctx),
                                modifier,
                            ))
                        }
                    }
                    _ => None,
                };
                match alt_projectile {
                    Some(alt_projectile) => Some(vec![alt_projectile]),
                    None => None,
                }
            }
            _ => None,
        };
        match projectiles {
            Some(projectiles) => {
                self.projectiles.extend(projectiles);
            }
            None => (),
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        let is_space_down = ctx.keyboard.pressed_keys().iter().find(|key| {
            if let KeyCode::Space = key {
                return true;
            }
            false
        });
        if is_space_down.is_some() {}
        match input.keycode {
            Some(key) => {
                match key {
                    KeyCode::Escape => {
                        // Exit the game when the escape key is pressed
                        ctx.request_quit();
                    }
                    _ => {
                        self.keys_pressed.insert(key);
                    }
                }
            }
            None => (),
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, key_input: KeyInput) -> Result<(), GameError> {
        if key_input.keycode.is_none() {
            return Ok(());
        }
        self.keys_pressed.remove(
            &key_input
                .keycode
                .expect(
                    "Failed to get keycode from key input. This should never happen as the keycode is checked for None"
                ));
        Ok(())
    }
}

fn main() {
    let window_mode = conf::WindowMode::default().dimensions(1920.0, 1080.0);
    let window_setup = conf::WindowSetup::default()
        .title("Hello ggez")
        .vsync(true)
        .samples(NumSamples::Four);
    let mut c = conf::Conf::new();
    c.window_mode = window_mode;
    c.window_setup = window_setup;

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {path:?}");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ContextBuilder::new("hello_ggez", "awesome_person").add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb
        .default_conf(c)
        .build()
        .expect("Failed to build ggez context");

    let res = ctx.gfx.window().set_cursor_grab(CursorGrabMode::Confined);
    res.is_err().then(|| {
        println!(
            "Error setting cursor position: {:?} (you can ignore this error)",
            res
        )
    });
    ctx.gfx.window().set_cursor_icon(CursorIcon::Crosshair);
    let res = ctx
        .gfx
        .window()
        .set_cursor_position(LogicalPosition::new(860.0, 540.0));
    res.is_err().then(|| {
        println!(
            "Error setting cursor position: {:?} (you can ignore this error)",
            res
        )
    });
    ctx.gfx.window().set_window_level(WindowLevel::AlwaysOnTop);
    let player = create_player(900.0, 900.0, Color::WHITE, create_spaceship_mesh(&mut ctx));
    let enemy = create_enemy(
        900.0,
        100.0,
        Color::RED,
        create_enemy_spaceship_mesh(&mut ctx),
        None,
        None,
    );
    let assets = Assets::new(&mut ctx);
    let assets = match assets {
        Ok(assets) => assets,
        Err(e) => {
            println!("Error loading assets: {:?}", e);
            std::process::exit(1);
        }
    };
    let state = GameState {
        dt: Duration::new(0, 0),
        assets,
        player,
        enemy: vec![enemy],
        projectiles: vec![],
        keys_pressed: HashSet::new(),
        kills: 0,
        alt_cd: 0.0,
        game_state_data: std::collections::HashMap::new(),
    };

    event::run(ctx, event_loop, state);
}
