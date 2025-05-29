mod game_state;
mod helpers;
mod sprites;

use game_state::GameState;
use ggez::{
  ContextBuilder, GameResult,
  conf::{NumSamples, WindowMode, WindowSetup},
  event::run,
  winit::window::WindowButtons,
};
use helpers::screen;
use log::info;

fn main() -> GameResult {
  env_logger::init();
  info!("Starting game...");

  let builder = ContextBuilder::new("pong_rs", "JoseLion");
  let window_mode = WindowMode::default()
    .dimensions(screen::width(), screen::height())
    .maximized(false)
    .resizable(false);
  let window_setup = WindowSetup::default()
    .title("Pong")
    .vsync(true)
    .samples(NumSamples::Four);
  let (ctx, event_loop) = builder
    .window_mode(window_mode)
    .window_setup(window_setup)
    .build()?;
  let state = GameState::new(&ctx)?;

  ctx
    .gfx
    .window()
    .set_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE);

  run(ctx, event_loop, state)
}
