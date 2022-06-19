use std::fmt::Debug;

pub trait Shape: Debug {
  fn get_x(&self) -> f32;
  fn get_y(&self) -> f32;
  fn set_x(&mut self, x: f32);
  fn set_y(&mut self, y: f32);
  fn get_vertices(&self) -> Vec<f32>;
  fn get_elements(&self) -> Vec<i32>;
  fn get_corners(&self) -> i32;
}

