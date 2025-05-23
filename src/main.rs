mod game_context;
mod helpers;
mod sprites;

use game_context::GameContext;
use helpers::{screen, shapes::draw_circle_smooth, theme};
use log::info;
use macroquad::{
  color::WHITE,
  shapes::{draw_line, draw_rectangle},
  time::draw_fps,
  window::{Conf, clear_background, next_frame},
};

#[cfg(windows)]
fn enable_dpi_awareness() {
  use winapi::um::winuser::SetProcessDPIAware;

  unsafe { SetProcessDPIAware() };
}

fn window_conf() -> Conf {
  #[cfg(windows)]
  enable_dpi_awareness();

  Conf {
    window_title: String::from("Pong"),
    window_width: 1280,
    window_height: 800,
    sample_count: 16,
    window_resizable: false,
    ..Conf::default()
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  env_logger::init();
  info!("Starting game...");

  let mut game = GameContext::new();

  loop {
    clear_background(theme::GREEN_900);

    game.update();

    draw_rectangle(0.0, 0.0, screen::cx(), screen::height(), theme::GREEN_500);
    draw_circle_smooth(screen::cx(), screen::cy(), 150.0, theme::GREEN_100);
    draw_line(
      screen::cx(),
      0.0,
      screen::cx(),
      screen::height(),
      2.0,
      WHITE,
    );

    game.draw();

    if cfg!(debug_assertions) {
      draw_fps();
    }

    next_frame().await;
  }
}
