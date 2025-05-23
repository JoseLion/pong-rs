use crate::helpers::{screen, shapes::draw_rectangle_rounded};
use macroquad::{
  color::WHITE,
  input::{KeyCode, is_key_down},
  math::Rect,
  text::draw_text,
  time::get_frame_time,
};

pub enum Role {
  AI,
  Human,
}

pub struct Player {
  role: Role,
  score: u8,
  x: f32,
  y: f32,
}

impl Player {
  const HEIGHT: f32 = 120.0;
  const OFFSET: f32 = 10.0;
  const SPEED: f32 = 275.0;
  const WIDTH: f32 = 25.0;
  const SCORE_SIZE: f32 = 150.0;
  const SCORE_OFFSET: f32 = 120.0;

  pub fn new(role: Role) -> Self {
    let y = screen::cy() - (Self::HEIGHT / 2.0);
    let x = match role {
      Role::AI => screen::width() - Self::WIDTH - Self::OFFSET,
      Role::Human => Self::OFFSET,
    };

    Self {
      role,
      score: 0,
      x,
      y,
    }
  }

  pub fn cy(&self) -> f32 {
    self.y + (Self::HEIGHT / 2.0)
  }

  pub fn add_point(&mut self) {
    self.score += 1;
  }

  pub fn rect(&self) -> Rect {
    Rect::new(self.x, self.y, Self::WIDTH, Self::HEIGHT)
  }

  pub fn update(&mut self) {
    match self.role {
      Role::AI => {}
      Role::Human => {
        if is_key_down(KeyCode::Down) {
          self.move_down();
        }

        if is_key_down(KeyCode::Up) {
          self.move_up();
        }
      }
    }
  }

  pub fn draw(&self) {
    let score_pos = match self.role {
      Role::AI => screen::cx() + (screen::cx() / 2.0),
      Role::Human => screen::cx() / 2.0,
    };

    draw_rectangle_rounded(self.x, self.y, Self::WIDTH, Self::HEIGHT, 8.0, WHITE);
    draw_text(
      &self.score.to_string(),
      score_pos,
      Self::SCORE_OFFSET,
      Self::SCORE_SIZE,
      WHITE,
    );
  }

  pub fn move_down(&mut self) {
    let frame_time = get_frame_time();
    let velocity = Self::SPEED * frame_time;
    self.y = (self.y + velocity).min(screen::height() - Self::HEIGHT);
  }

  pub fn move_up(&mut self) {
    let frame_time = get_frame_time();
    let velocity = Self::SPEED * frame_time;
    self.y = (self.y - velocity).max(0.0);
  }
}
