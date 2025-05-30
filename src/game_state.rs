use crate::{
  helpers::{screen, shapes::check_collision_circle_rect, theme},
  sprites::{
    ball::{Ball, Direction},
    player::{Player, Role},
  },
};
use ggez::{
  Context, GameError, GameResult,
  event::EventHandler,
  glam::Vec2,
  graphics::{Canvas, Color, DrawMode, Mesh, Rect, Text},
};

pub struct GameState {
  courtyard: Mesh,
  midfield: Mesh,
  midline: Mesh,
  debug_fps: Text,
  ai: Player,
  human: Player,
  ball: Ball,
}

impl GameState {
  pub fn new(ctx: &Context) -> GameResult<Self> {
    let courtyard = Mesh::new_rectangle(
      ctx,
      DrawMode::fill(),
      Rect::new(0.0, 0.0, screen::cx(), screen::height()),
      theme::GREEN_500,
    )?;
    let midfield = Mesh::new_circle(
      ctx,
      DrawMode::fill(),
      Vec2::ZERO,
      150.0,
      0.01,
      theme::GREEN_100,
    )?;
    let midline = Mesh::new_line(
      ctx,
      &[Vec2::ZERO, Vec2::new(0.0, screen::height())],
      2.0,
      Color::WHITE,
    )?;
    let debug_fps = Text::new("FPS: 0").set_scale(32.0).to_owned();

    Ok(Self {
      courtyard,
      midfield,
      midline,
      debug_fps,
      ai: Player::new(ctx, Role::AI)?,
      human: Player::new(ctx, Role::Human)?,
      ball: Ball::new(ctx)?,
    })
  }

  fn move_ai(&mut self, ctx: &mut Context) {
    match self.ball.y() > self.ai.cy() {
      true => self.ai.move_down(ctx),
      false => self.ai.move_up(ctx),
    }
  }

  fn handle_collisions(&mut self) {
    if check_collision_circle_rect(&self.ball.circle(), &self.ai.rect()) {
      self.ball.bounce(Direction::Left);
    }

    if check_collision_circle_rect(&self.ball.circle(), &self.human.rect()) {
      self.ball.bounce(Direction::Right);
    }
  }

  fn handle_score(&mut self) {
    if self.ball.x() - Ball::RADIUS <= 0.0 {
      self.ai.add_point();
      self.ball.respawn();
    }

    if self.ball.x() + Ball::RADIUS >= screen::width() {
      self.human.add_point();
      self.ball.respawn();
    }
  }
}

impl EventHandler<GameError> for GameState {
  fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
    self.handle_collisions();
    self.handle_score();
    self.move_ai(ctx);

    self.ball.update(ctx);
    self.human.update(ctx);
    self.ai.update(ctx);

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
    let mut canvas = Canvas::from_frame(ctx, theme::GREEN_900);

    canvas.draw(&self.courtyard, Vec2::ZERO);
    canvas.draw(&self.midfield, Vec2::new(screen::cx(), screen::cy()));
    canvas.draw(&self.midline, Vec2::new(screen::cx(), 0.0));

    self.ball.draw(&mut canvas);
    self.human.draw(&mut canvas)?;
    self.ai.draw(&mut canvas)?;

    if cfg!(debug_assertions) {
      let fps = format!("FPS: {:.0}", ctx.time.fps());

      if self.debug_fps.contents() != fps {
        self.debug_fps.clear();
        self.debug_fps.add(&fps);
      }

      canvas.draw(&self.debug_fps, Vec2::new(10.0, 10.0));
    }

    canvas.finish(ctx)?;
    Ok(())
  }
}
