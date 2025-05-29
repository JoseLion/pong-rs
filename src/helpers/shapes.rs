use ggez::graphics::Rect;

pub struct Circle {
  pub x: f32,
  pub y: f32,
  pub r: f32,
}

pub fn check_collision_circle_rect(circle: &Circle, rect: &Rect) -> bool {
  let closest_x = circle.x.clamp(rect.x, rect.x + rect.w);
  let closest_y = circle.y.clamp(rect.y, rect.y + rect.h);
  let dx = circle.x - closest_x;
  let dy = circle.y - closest_y;

  dx.powi(2) + dy.powi(2) < circle.r.powi(2)
}
