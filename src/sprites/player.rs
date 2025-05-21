use crate::helpers::{screen, shapes::draw_rectangle_rounded};
use macroquad::{
  color::WHITE,
  input::{KeyCode, is_key_down},
  math::Rect,
  time::get_frame_time,
};

pub enum Role {
  AI,
  Human,
}

pub struct Player {
  role: Role,
  x: f32,
  y: f32,
}

impl Player {
  pub const HEIGHT: f32 = 120.0;
  const OFFSET: f32 = 10.0;
  const SPEED: f32 = 275.0;
  const WIDTH: f32 = 25.0;

  pub fn new(role: Role) -> Self {
    let y = screen::cy() - (Self::HEIGHT / 2.0);
    let x = match role {
      Role::AI => screen::width() - Self::WIDTH - Self::OFFSET,
      Role::Human => Self::OFFSET,
    };

    Self { role, x, y }
  }

  pub fn y(&self) -> f32 {
    self.y
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
    draw_rectangle_rounded(self.x, self.y, Self::WIDTH, Self::HEIGHT, 8.0, WHITE);
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
