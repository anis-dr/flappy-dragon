use bracket_lib::prelude::*;

struct Player {
  x: i32,
  y: i32,
  velocity: f32,
}

impl Player {
  fn new(x: i32, y: i32) -> Self {
    Player {
      x,
      y,
      velocity: 0.0,
    }
  }

  fn render(&mut self, ctx: &mut BTerm) {
    ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
  }

  fn gravity_and_move(&mut self) {
    if self.velocity < 2.0 {
      self.velocity += 0.2;
    }
    self.y += self.velocity as i32;
    self.x += 1;
    if self.y < 0 {
      self.y = 0;
    }
  }

  fn flap(&mut self) { 
    self.velocity = -2.0;
  }
}
