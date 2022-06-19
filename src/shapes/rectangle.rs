use crate::general::color::Color;

use super::shape::Shape;

#[derive(Debug)]
pub struct Rectangle {
  pub width: f32,
  pub height: f32,
  pub x: f32,
  pub y: f32,
  pub color: Color,
}


impl Shape for Rectangle {

  fn get_x(&self) -> f32 {
      self.x
  }

  fn get_y(&self) -> f32 {
      self.x
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

  fn get_corners(&self) -> i32 {
    return 4;
  }

}


