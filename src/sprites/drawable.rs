use crate::gl_utils::gl_texture::Texture;
use crate::shapes::shape::Shape;

pub trait Drawable<'a> {
  fn get_shape_ptr(&'a self) -> &'a dyn Shape;
  fn get_texture_ptr(&'a self) -> &'a Texture;
}
