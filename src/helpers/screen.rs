use macroquad::window::{screen_height, screen_width};

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
