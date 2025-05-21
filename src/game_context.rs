use crate::{
  helpers::shapes::check_collision_circle_rect,
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
    self.check_collisions();
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
    let cy = self.ai.y() + (Player::HEIGHT / 2.0);

    if self.ball.y() > cy {
      self.ai.move_down();
    } else {
      self.ai.move_up();
    }
  }

  fn check_collisions(&mut self) {
    if check_collision_circle_rect(self.ball.circle(), self.ai.rect()) {
      self.ball.bounce(Direction::Left);
    }

    if check_collision_circle_rect(self.ball.circle(), self.human.rect()) {
      self.ball.bounce(Direction::Right);
    }
  }
}
