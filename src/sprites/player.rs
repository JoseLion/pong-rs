use ggez::{
  Context, GameError, GameResult,
  glam::Vec2,
  graphics::{Canvas, Color, DrawMode, Mesh, Rect, Text, TextFragment},
  input::keyboard::KeyCode,
};

use crate::helpers::screen;

pub enum Role {
  AI,
  Human,
}

pub struct Player {
  paddle: Mesh,
  score_text: Text,
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
  const SCORE_OFFSET: f32 = 20.0;

  pub fn new(ctx: &Context, role: Role) -> GameResult<Self> {
    let paddle = Mesh::new_rounded_rectangle(
      ctx,
      DrawMode::fill(),
      Rect::new(0.0, 0.0, Self::WIDTH, Self::HEIGHT),
      8.0,
      Color::WHITE,
    )?;
    let fragment = TextFragment::new("0").color(Color::WHITE);
    let score_text = Text::new(fragment).set_scale(Self::SCORE_SIZE).to_owned();
    let y = screen::cy() - (Self::HEIGHT / 2.0);
    let x = match role {
      Role::AI => screen::width() - Self::WIDTH - Self::OFFSET,
      Role::Human => Self::OFFSET,
    };

    Ok(Self {
      paddle,
      score_text,
      role,
      score: 0,
      x,
      y,
    })
  }

  pub fn cy(&self) -> f32 {
    self.y + (Self::HEIGHT / 2.0)
  }

  pub fn add_point(&mut self) {
    self.score += 1;

    let frag = TextFragment::new(self.score.to_string()).color(Color::WHITE);
    self.score_text.clear();
    self.score_text.add(frag);
  }

  pub fn rect(&self) -> Rect {
    Rect::new(self.x, self.y, Self::WIDTH, Self::HEIGHT)
  }

  pub fn update(&mut self, ctx: &mut Context) {
    match self.role {
      Role::AI => {}
      Role::Human => {
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
          self.move_down(ctx);
        }

        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
          self.move_up(ctx);
        }
      }
    }
  }

  pub fn draw(&self, canvas: &mut Canvas) -> Result<(), GameError> {
    let score_pos = match self.role {
      Role::AI => screen::cx() + (screen::cx() / 2.0),
      Role::Human => screen::cx() / 2.0,
    };

    canvas.draw(&self.paddle, Vec2::new(self.x, self.y));
    canvas.draw(&self.score_text, Vec2::new(score_pos, Self::SCORE_OFFSET));
    Ok(())
  }

  pub fn move_down(&mut self, ctx: &mut Context) {
    let delta = ctx.time.delta().as_secs_f32().clamp(0.0, 1.0 / 30.0);
    let velocity = Self::SPEED * delta;

    self.y = (self.y + velocity).min(screen::height() - Self::HEIGHT);
  }

  pub fn move_up(&mut self, ctx: &mut Context) {
    let delta = ctx.time.delta().as_secs_f32().clamp(0.0, 1.0 / 30.0);
    let velocity = Self::SPEED * delta;

    self.y = (self.y - velocity).max(0.0);
  }
}
