mod actors;
mod asset_manager;
pub mod behaviors;
mod main_game_state;

use crate::asset_manager::Assets;
use crate::behaviors::enemy_ai::model::EnemyAi;
use crate::behaviors::model::BehaviorTreeTrait;
use crate::main_game_state::build_main_game_state;
use ggez::audio::SoundSource;
use ggez::conf::NumSamples;
use ggez::graphics::Drawable;
use ggez::input::mouse::CursorIcon;
use ggez::winit::dpi::LogicalPosition;
use ggez::winit::window::{CursorGrabMode, WindowLevel};
use ggez::{conf, ContextBuilder};
use rand::prelude::*;
use std::{env, path};

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
    let assets = Assets::new(&mut ctx);
    let assets = match assets {
        Ok(assets) => assets,
        Err(e) => {
            println!("Error loading assets: {:?}", e);
            std::process::exit(1);
        }
    };
    let res = build_main_game_state(ctx, event_loop, assets);
    match res {
        Ok(_) => println!("Exited cleanly"),
        Err(e) => println!("Error running game: {:?}", e),
    }
}
