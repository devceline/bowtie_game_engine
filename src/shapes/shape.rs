pub trait Shape {
  fn get_vertices(&self) -> Vec<f32>;
  fn get_elements(&self) -> Vec<i32>;
}
