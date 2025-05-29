use crate::helpers::{screen, shapes::Circle};
use ggez::{
  Context, GameResult,
  glam::Vec2,
  graphics::{Canvas, Color, DrawMode, Mesh},
};
use rand::Rng;

#[derive(Clone, Copy)]
pub enum Direction {
  Down,
  Left,
  Right,
  Up,
}

pub struct Ball {
  mesh: Mesh,
  x: f32,
  y: f32,
  vx: f32,
  vy: f32,
}

impl Ball {
  pub const RADIUS: f32 = 20.0;
  const SPEED: f32 = 375.0;

  pub fn new(ctx: &Context) -> GameResult<Self> {
    let mesh = Mesh::new_circle(
      ctx,
      DrawMode::fill(),
      Vec2::ZERO,
      Self::RADIUS,
      0.1,
      Color::WHITE,
    )?;

    Ok(Ball {
      mesh,
      x: screen::cx(),
      y: screen::cy(),
      vx: Self::random_speed(),
      vy: Self::random_speed(),
    })
  }

  pub fn x(&self) -> f32 {
    self.x
  }

  pub fn y(&self) -> f32 {
    self.y
  }

  pub fn circle(&self) -> Circle {
    Circle {
      x: self.x,
      y: self.y,
      r: Ball::RADIUS,
    }
  }

  pub fn respawn(&mut self) {
    self.x = screen::cx();
    self.y = screen::cy();
    self.vx = Self::random_speed();
    self.vy = Self::random_speed();
  }

  pub fn update(&mut self, ctx: &mut Context) {
    let delta = ctx.time.delta().as_secs_f32().clamp(0.0, 1.0 / 30.0);

    self.x += self.vx * delta;
    self.y += self.vy * delta;

    if self.y + Self::RADIUS >= screen::height() {
      self.bounce(Direction::Up);
    }

    if self.y - Self::RADIUS < 0.0 {
      self.bounce(Direction::Down);
    }
  }

  pub fn draw(&self, canvas: &mut Canvas) {
    canvas.draw(&self.mesh, Vec2::new(self.x, self.y));
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
    match rand::rng().random_range(0..10) % 2 == 0 {
      true => Self::SPEED,
      false => -Self::SPEED,
    }
  }
}
