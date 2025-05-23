use crate::helpers::{screen, shapes::draw_circle_smooth};
use macroquad::{color::WHITE, math::Circle, prelude::rand, time::get_frame_time};

pub enum Direction {
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
  pub const RADIUS: f32 = 20.0;
  const SPEED: f32 = 375.0;

  pub fn new() -> Self {
    Ball {
      x: screen::cx(),
      y: screen::cy(),
      vx: Self::random_speed(),
      vy: Self::random_speed(),
    }
  }

  pub fn x(&self) -> f32 {
    self.x
  }

  pub fn y(&self) -> f32 {
    self.y
  }

  pub fn circle(&self) -> Circle {
    Circle::new(self.x, self.y, Ball::RADIUS)
  }

  pub fn update(&mut self) {
    let frame_time = get_frame_time();

    self.x += self.vx * frame_time;
    self.y += self.vy * frame_time;

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

  pub fn bounce(&mut self, direction: Direction) {
    match direction {
      Direction::Down => self.vy = Self::SPEED,
      Direction::Left => self.vx = -Self::SPEED,
      Direction::Right => self.vx = Self::SPEED,
      Direction::Up => self.vy = -Self::SPEED,
    }
  }

  fn random_speed() -> f32 {
    match rand::gen_range(0, 10) % 2 == 0 {
      true => Self::SPEED,
      false => -Self::SPEED,
    }
  }
}
