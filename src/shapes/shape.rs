use std::fmt::Debug;

use crate::general::color::Color;

pub trait Shape: Debug {
  fn get_x(&self) -> f32;
  fn get_y(&self) -> f32;
  fn set_x(&mut self, x: f32);
  fn set_y(&mut self, y: f32);
  fn get_width(&self) -> f32;
  fn get_height(&self) -> f32;
  fn set_height(&mut self, height: f32);
  fn set_width(&mut self, width: f32);
  fn get_color(&self) -> Color;
  fn set_color(&mut self, color: Color);
  fn get_coordinate_corners(&self) -> [[f32; 2]; 4];
  fn get_texture_corners(&self) -> [[f32; 2]; 4];
}
