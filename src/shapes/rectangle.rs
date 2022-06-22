use crate::general::color::Color;

use super::shape::Shape;

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
  pub width: f32,
  pub height: f32,
  pub x: f32,
  pub y: f32,
  pub color: Color,
  texture_corners: [[f32; 2]; 4],
}

impl Rectangle {
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
      texture_corners:  [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
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

  fn flip_texture_corners_x(&mut self) {
    let mut new_texture_corners = self.texture_corners.to_owned();

    for row in 0..4 {
      new_texture_corners[row][0] = 1.0 - self.texture_corners[row][0];
    }
    self.texture_corners = new_texture_corners;
  }

  fn flip_texture_corners_y(&mut self) {
    let mut new_texture_corners = self.texture_corners.to_owned();

    for row in 0..4 {
      new_texture_corners[row][1] = 1.0 - self.texture_corners[row][1];
    }
    self.texture_corners = new_texture_corners;
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
