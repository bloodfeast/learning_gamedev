mod actors;
mod asset_manager;
pub mod behaviors;
mod main_game_state;
mod main_menu_game_state;

use crate::asset_manager::Assets;
use crate::behaviors::enemy_ai::model::EnemyAi;
use crate::behaviors::model::BehaviorTreeTrait;
use crate::main_game_state::build_main_game_state;
use ggez::audio::SoundSource;
use ggez::conf::{NumSamples, WindowMode, WindowSetup};
use ggez::graphics::Drawable;
use ggez::input::mouse::CursorIcon;
use ggez::winit::dpi::LogicalPosition;
use ggez::winit::window::{CursorGrabMode, WindowLevel};
use ggez::{conf, event, ContextBuilder};
use rand::prelude::*;
use std::ops::Deref;
use std::path::PathBuf;
use std::{env, path};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum State {
    MainMenu,
    MainGame,
}
#[derive(Debug, Clone)]
pub struct MetaState {
    pub window_mode: WindowMode,
    pub window_setup: WindowSetup,
    pub cursor_grab: CursorGrabMode,
    pub cursor_icon: CursorIcon,
    pub resources_dir: PathBuf,
    pub current_state: State,
}

impl MetaState {
    pub fn new(
        window_mode: Option<WindowMode>,
        window_setup: Option<WindowSetup>,
        vsync: Option<bool>,
        samples: Option<NumSamples>,
        cursor_grab: Option<CursorGrabMode>,
        cursor_icon: Option<CursorIcon>,
        resources_dir: PathBuf,
    ) -> Self {
        let window_mode = window_mode.unwrap_or(WindowMode::default().dimensions(1920.0, 1080.0));
        let mut window_setup = window_setup.unwrap_or(WindowSetup::default().title("Hello ggez"));
        window_setup.vsync = vsync.unwrap_or(true);
        window_setup.samples = samples.unwrap_or(NumSamples::Four);
        let cursor_grab = cursor_grab.unwrap_or(CursorGrabMode::Confined);
        let cursor_icon = cursor_icon.unwrap_or(CursorIcon::Crosshair);

        MetaState {
            window_mode,
            window_setup,
            cursor_grab,
            cursor_icon,
            resources_dir,
            current_state: State::MainMenu,
        }
    }

    pub fn context_builder(&self) -> ContextBuilder {
        let mut c = conf::Conf::new();
        c.window_mode = self.window_mode.clone();
        c.window_setup = self.window_setup.clone();
        ContextBuilder::new("hello_ggez", "awesome_person")
            .add_resource_path(self.resources_dir.clone())
            .default_conf(c)
    }

    pub fn set_cursor_grab(&self, ctx: &mut ggez::Context) {
        let res = ctx.gfx.window().set_cursor_grab(self.cursor_grab);
        res.is_err().then(|| {
            println!(
                "Error setting cursor position: {:?} (you can ignore this error)",
                res
            )
        });
    }

    pub fn set_cursor_position(&self, ctx: &mut ggez::Context) {
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
    }

    pub fn set_cursor_icon(&self, ctx: &mut ggez::Context) {
        ctx.gfx.window().set_cursor_icon(self.cursor_icon);
    }

    pub fn set_window_level(&self, ctx: &mut ggez::Context, level: Option<WindowLevel>) {
        let level = level.unwrap_or(WindowLevel::AlwaysOnTop);
        ctx.gfx.window().set_window_level(level);
    }

    pub fn set_assets(&self, ctx: &mut ggez::Context) -> Assets {
        let assets = Assets::new(ctx);
        let assets = match assets {
            Ok(assets) => assets,
            Err(e) => {
                println!("Error loading assets: {:?}", e);
                std::process::exit(1);
            }
        };
        assets
    }

    pub fn update_game_state(&mut self, next_state: State) {
        self.current_state = next_state;
    }
}

fn main() {
    let window_mode = WindowMode::default().dimensions(1920.0, 1080.0);
    let window_setup = WindowSetup::default().title("Hello ggez");
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {path:?}");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let mut meta_state = MetaState::new(
        Some(window_mode),
        Some(window_setup),
        Some(true),
        Some(NumSamples::Four),
        Some(CursorGrabMode::Confined),
        Some(CursorIcon::Crosshair),
        resource_dir,
    );

    let (mut ctx, event_loop) = meta_state
        .context_builder()
        .build()
        .expect("Failed to build ggez context");

    meta_state.set_window_level(&mut ctx, None);
    meta_state.set_cursor_grab(&mut ctx);
    meta_state.set_cursor_position(&mut ctx);
    meta_state.set_cursor_icon(&mut ctx);
    let assets = meta_state.set_assets(&mut ctx);
    let (ctx, game_state) = build_main_game_state(ctx, assets);
    event::run(ctx, event_loop, game_state);
}
