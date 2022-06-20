use crate::general::color::Color;

use super::shape::Shape;

#[derive(Debug)]
pub struct Rectangle {
  pub width: f32,
  pub height: f32,
  pub x: f32,
  pub y: f32,
  pub color: Color,
  texture_corners: [[f32; 2]; 4],
}

impl Rectangle {
  const TEX_CORDS_CORNERS: [[f32; 2]; 4] =
    [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

  pub fn new(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
  ) -> Rectangle {
    Rectangle {
      x,
      y,
      width,
      height,
      color,
      texture_corners: Rectangle::TEX_CORDS_CORNERS,
    }
  }
}

impl Shape for Rectangle {
  fn get_x(&self) -> f32 {
    self.x
  }

  fn get_y(&self) -> f32 {
    self.y
  }

  fn set_x(&mut self, x: f32) {
    self.x = x;
  }

  fn set_y(&mut self, y: f32) {
    self.y = y;
  }

  fn get_width(&self) -> f32 {
    self.width
  }

  fn get_height(&self) -> f32 {
    self.height
  }

  fn set_height(&mut self, height: f32) {
    self.height = height;
  }

  fn set_width(&mut self, width: f32) {
    self.width = width;
  }

  fn get_color(&self) -> Color {
    self.color
  }

  fn set_color(&mut self, color: Color) {
    self.color = color;
  }

  fn get_texture_corners(&self) -> [[f32; 2]; 4] {
    self.texture_corners
  }

  fn get_coordinate_corners(&self) -> [[f32; 2]; 4] {
    [
      [self.x, self.y],
      [self.x + self.width, self.y],
      [self.x + self.width, self.y - self.height],
      [self.x, self.y - self.height],
    ]
  }
}
