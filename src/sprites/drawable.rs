use std::fmt::Debug;

use crate::gl_utils::gl_texture::Texture;
use crate::shapes::shape::Shape;

pub trait Drawable<'a>: Debug {
  fn get_shape_ptr(&'a self) -> &'a dyn Shape;
  fn get_texture_ptr(&'a self) -> &'a Texture;
  fn get_vertices(&self) -> Vec<f32>;
  fn get_elements(&self) -> Vec<i32>;
}
