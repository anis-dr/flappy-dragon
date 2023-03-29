use bracket_lib::prelude::*;
use state::State;

mod player;
mod state;

fn main() -> BError {
  let context = BTermBuilder::simple80x50()
    .with_title("Flappy Dragon")
    .build()?;
  main_loop(context, State::new())
}
