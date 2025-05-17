mod helpers;

use helpers::prelude::*;
use helpers::theme;
use log::info;
use macroquad::prelude::*;

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

  loop {
    clear_background(theme::GREEN_900);

    draw_rectangle(0.0, 0.0, screen::cx(), screen::h(), theme::GREEN_500);
    draw_circle_smooth(screen::cx(), screen::cy(), 150.0, theme::GREEN_100);
    draw_line(screen::cx(), 0.0, screen::cx(), screen::h(), 2.0, WHITE);

    next_frame().await;
  }
}
