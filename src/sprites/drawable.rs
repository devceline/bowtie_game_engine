use std::fmt::Debug;

use crate::gl_utils::gl_texture::Texture;
use crate::shapes::shape::Shape;

/*
 * Trait interface that abstracts sprites, in case we want to pass
 * Something more complicated than a simple sprite. Eg, a playable character
 */
pub trait Drawable<'a>: Debug {
  fn get_shape_ptr(&'a self) -> &'a dyn Shape;
  fn get_texture_ptr(&'a self) -> &'a Texture;
  fn get_vertices(&self) -> Vec<f32>;
  fn get_elements(&self) -> Vec<i32>;
}
