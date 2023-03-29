use bracket_lib::prelude::*;

pub struct State {
  mode: GameMode,
}

impl State {
  pub(crate) fn new() -> Self {
    Self {
      mode: GameMode::Menu,
    }
  }

  fn play(&mut self, ctx: &mut BTerm) {
    // TODO: Fill in this stub later
    self.mode = GameMode::End;
  }

  fn restart(&mut self) {
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
