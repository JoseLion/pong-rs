use macroquad::{
  color::Color,
  math::{Circle, Rect},
  shapes::{draw_poly, draw_rectangle},
};

const SEGMENT_FACTOR: f32 = 0.75;

pub fn draw_circle_smooth(x: f32, y: f32, radius: f32, color: Color) {
  let sides = (radius * SEGMENT_FACTOR).ceil().clamp(24.0, 96.0) as u8;

  draw_poly(x, y, sides, radius, 0.0, color);
}

pub fn draw_rectangle_rounded(x: f32, y: f32, width: f32, height: f32, radius: f32, color: Color) {
  let r = radius.min(width / 2.0).min(height / 2.0);
  let d = r * 2.0;

  draw_rectangle(x + r, y, width - d, height, color);
  draw_rectangle(x, y + r, r, height - d, color);
  draw_rectangle(x + width - r, y + r, r, height - d, color);

  draw_circle_smooth(x + r, y + r, r, color);
  draw_circle_smooth(x + width - r, y + r, r, color);
  draw_circle_smooth(x + r, y + height - r, r, color);
  draw_circle_smooth(x + width - r, y + height - r, r, color);
}

pub fn check_collision_circle_rect(circle: Circle, rect: Rect) -> bool {
  let closest_x = circle.x.clamp(rect.x, rect.x + rect.w);
  let closest_y = circle.y.clamp(rect.y, rect.y + rect.h);
  let dx = circle.x - closest_x;
  let dy = circle.y - closest_y;

  dx.powi(2) + dy.powi(2) < circle.r.powi(2)
}
