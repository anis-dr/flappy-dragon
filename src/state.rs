use bracket_lib::prelude::*;

use crate::player::Player;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

pub struct State {
  player: Player,
  frame_time: f32,
  mode: GameMode,
}

impl State {
  pub(crate) fn new() -> Self {
    Self {
      player: Player::new(5, 25),
      frame_time: 0.0,
      mode: GameMode::Menu,
    }
  }

  fn play(&mut self, ctx: &mut BTerm) {
    ctx.cls_bg(NAVY);
    self.frame_time += ctx.frame_time_ms;
    if self.frame_time > FRAME_DURATION {
      self.frame_time = 0.0;

      self.player.gravity_and_move();
    }
    if let Some(VirtualKeyCode::Space) = ctx.key {
      self.player.flap();
    }
    self.player.render(ctx);
    ctx.print(0, 0, "Press SPACE to flap.");
    if self.player.y > SCREEN_HEIGHT {
      self.mode = GameMode::End;
    }
  }

  fn restart(&mut self) {
    self.player = Player::new(5, 25);
    self.frame_time = 0.0;
    self.mode = GameMode::Playing;
  }

  fn menu_actions(&mut self, ctx: &mut BTerm) {
    if let Some(key) = ctx.key {
      match key {
        VirtualKeyCode::P => self.restart(),
        VirtualKeyCode::Q => ctx.quitting = true,
        _ => {}
      }
    }
  }

  fn main_menu(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    ctx.print_centered(5, "Welcome to Flappy Dragon");
    ctx.print_centered(8, "(P) Play Game");
    ctx.print_centered(9, "(Q) Quit Game");

    self.menu_actions(ctx);
  }

  fn dead(&mut self, ctx: &mut BTerm) {
    ctx.cls();
    ctx.print_centered(5, "You are dead!");
    ctx.print_centered(8, "(P) Play Again");
    ctx.print_centered(9, "(Q) Quit Game");

    self.menu_actions(ctx);
  }
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut BTerm) {
    match self.mode {
      GameMode::Menu => self.main_menu(ctx),
      GameMode::End => self.dead(ctx),
      GameMode::Playing => self.play(ctx),
    }
  }
}

enum GameMode {
  Menu,
  Playing,
  End,
}
