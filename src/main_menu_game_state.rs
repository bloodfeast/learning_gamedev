use crate::MetaState;
use ggez::graphics::{Canvas, Color, Drawable, Text, TextFragment};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{event, Context, GameError, GameResult};
use std::cmp::PartialEq;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MenuOption {
    StartGame,
    Exit,
}

pub struct MainMenuState {
    selected_menu_option: MenuOption,
    menu_options: Vec<MenuOption>,
    meta_state: MetaState,
}
impl MainMenuState {
    pub fn new(menu_options: Vec<MenuOption>, meta_state: &MetaState) -> GameResult<MainMenuState> {
        let selected_menu_option = MenuOption::StartGame;
        Ok(MainMenuState {
            selected_menu_option,
            menu_options,
            meta_state: meta_state.clone(),
        })
    }
}

impl event::EventHandler<GameError> for MainMenuState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        for (i, menu_option) in self.menu_options.iter().enumerate() {
            let y_pos = 200.0 + (i as f32 * 200.0);
            match menu_option {
                MenuOption::StartGame => {
                    let mut start_game_text = TextFragment::new("Start Game");
                    start_game_text.color = Some(Color::new(1.0, 1.0, 1.0, 1.0));
                    if self.selected_menu_option == MenuOption::StartGame {
                        start_game_text.color = Some(Color::new(0.8, 0.8, 0.0, 1.0));
                    }
                    let mut start_game = Text::new(start_game_text);
                    start_game.set_scale(50.0);
                    start_game.draw(&mut canvas, [900.0, y_pos]);
                }
                MenuOption::Exit => {
                    let mut exit_text = TextFragment::new("Exit");
                    exit_text.color = Some(Color::new(1.0, 1.0, 1.0, 1.0));

                    if self.selected_menu_option == MenuOption::Exit {
                        exit_text.color = Some(Color::new(0.8, 0.8, 0.0, 1.0));
                    }
                    let mut exit = Text::new(exit_text);
                    exit.set_scale(50.0);
                    exit.draw(&mut canvas, [900.0, y_pos]);
                }
            }
        }

        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        key_input: KeyInput,
        _repeat: bool,
    ) -> Result<(), GameError> {
        match key_input.keycode {
            Some(KeyCode::Up) | Some(KeyCode::W) => {
                let selected_index = self
                    .menu_options
                    .iter()
                    .position(|x| x == &self.selected_menu_option)
                    .unwrap();
                let new_index = if selected_index == 0 {
                    self.menu_options.len() - 1
                } else {
                    selected_index - 1
                };
                self.selected_menu_option = self.menu_options[new_index].clone();
                Ok(())
            }
            Some(KeyCode::Down) | Some(KeyCode::S) => {
                let selected_index = self
                    .menu_options
                    .iter()
                    .position(|x| x == &self.selected_menu_option)
                    .unwrap();
                let new_index = if selected_index == self.menu_options.len() - 1 {
                    0
                } else {
                    selected_index + 1
                };
                self.selected_menu_option = self.menu_options[new_index].clone();
                Ok(())
            }
            Some(KeyCode::Return) | Some(KeyCode::Space) => match self.selected_menu_option {
                MenuOption::StartGame => {
                    println!("Starting game");
                    Ok(())
                }
                MenuOption::Exit => {
                    println!("Exiting game");
                    ctx.request_quit();
                    Ok(())
                }
            },
            _ => Ok(()),
        }
    }
}
