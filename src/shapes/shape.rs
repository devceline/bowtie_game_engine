use std::fmt::Debug;

pub trait Shape: Debug {
  fn get_vertices(&self) -> Vec<f32>;
  fn get_elements(&self) -> Vec<i32>;
  fn get_corners(&self) -> i32;
  fn move_right(&mut self, amount: f32) -> bool;
  fn move_left(&mut self, amount: f32) -> bool;
  fn move_up(&mut self, amount: f32) -> bool;
  fn move_down(&mut self, amount: f32) -> bool;
}

