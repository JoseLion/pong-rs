use crate::{
  helpers::{screen, shapes::check_collision_circle_rect},
  sprites::{
    ball::{Ball, Direction},
    player::{Player, Role},
  },
};

pub struct GameContext {
  ai: Player,
  human: Player,
  ball: Ball,
}

impl GameContext {
  pub fn new() -> Self {
    Self {
      ai: Player::new(Role::AI),
      human: Player::new(Role::Human),
      ball: Ball::new(),
    }
  }

  pub fn update(&mut self) {
    self.handle_collisions();
    self.handle_score();
    self.move_ai();

    self.ball.update();
    self.human.update();
    self.ai.update();
  }

  pub fn draw(&self) {
    self.ball.draw();
    self.human.draw();
    self.ai.draw();
  }

  fn move_ai(&mut self) {
    match self.ball.y() > self.ai.cy() {
      true => self.ai.move_down(),
      false => self.ai.move_up(),
    }
  }

  fn handle_collisions(&mut self) {
    if check_collision_circle_rect(self.ball.circle(), self.ai.rect()) {
      self.ball.bounce(Direction::Left);
    }

    if check_collision_circle_rect(self.ball.circle(), self.human.rect()) {
      self.ball.bounce(Direction::Right);
    }
  }

  fn handle_score(&mut self) {
    if self.ball.x() - Ball::RADIUS <= 0.0 {
      self.ai.add_point();
      self.ball = Ball::new();
    }

    if self.ball.x() + Ball::RADIUS >= screen::width() {
      self.human.add_point();
      self.ball = Ball::new();
    }
  }
}
