use std::fmt::Debug;

use crate::gl_utils::shader_creator::ShaderProgram;

/// Trait interface that abstracts sprites, in case we want to pass
/// Something more complicated than a simple sprite. Eg, a playable character
pub trait Drawable<'a>: Debug {
  fn load_texture(&'a self) -> ();
  fn set_texture_uniform(&'a self, program: &ShaderProgram) -> ();
  fn get_corner_count(&'a self) -> i32;
  fn get_vertices(&self) -> Vec<f32>;
  fn get_elements(&self) -> Vec<i32>;
}
