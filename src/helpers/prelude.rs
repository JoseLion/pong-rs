use macroquad::prelude::*;

pub mod screen {
  use macroquad::prelude::*;

  pub fn width() -> f32 {
    screen_width()
  }

  pub fn height() -> f32 {
    screen_height()
  }

  pub fn cx() -> f32 {
    width() / 2.0
  }

  pub fn cy() -> f32 {
    height() / 2.0
  }
}

const SEGMENT_FACTOR: f32 = 0.75;

pub fn draw_circle_smooth(x: f32, y: f32, radius: f32, color: Color) {
  let sides = (radius * SEGMENT_FACTOR).ceil().clamp(24.0, 96.0) as u8;

  draw_poly(x, y, sides, radius, 0.0, color);
}
