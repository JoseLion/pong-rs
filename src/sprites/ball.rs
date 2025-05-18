use crate::helpers::prelude::*;
use macroquad::{color::WHITE, time::get_frame_time};

enum Direction {
  Down,
  Left,
  Right,
  Up,
}

pub struct Ball {
  x: f32,
  y: f32,
  vx: f32,
  vy: f32,
}

impl Ball {
  const RADIUS: f32 = 20.0;
  const SPEED: f32 = 350.0;

  pub fn new() -> Self {
    Ball {
      x: screen::cx(),
      y: screen::cy(),
      vx: Self::SPEED,
      vy: Self::SPEED,
    }
  }

  pub fn update(&mut self) {
    let frame_time = get_frame_time();

    self.x += self.vx * frame_time;
    self.y += self.vy * frame_time;

    if self.x + Self::RADIUS >= screen::width() {
      self.bounce(Direction::Left);
    }

    if self.x - Self::RADIUS < 0.0 {
      self.bounce(Direction::Right);
    }

    if self.y + Self::RADIUS >= screen::height() {
      self.bounce(Direction::Up)
    }

    if self.y - Self::RADIUS < 0.0 {
      self.bounce(Direction::Down);
    }
  }

  pub fn draw(&self) {
    draw_circle_smooth(self.x, self.y, Self::RADIUS, WHITE);
  }

  fn bounce(&mut self, direction: Direction) {
    match direction {
      Direction::Down => self.vy = Self::SPEED,
      Direction::Left => self.vx = -Self::SPEED,
      Direction::Right => self.vx = Self::SPEED,
      Direction::Up => self.vy = -Self::SPEED,
    }
  }
}
