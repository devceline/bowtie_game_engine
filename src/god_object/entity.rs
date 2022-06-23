use crate::sprites::drawable::Drawable;

pub trait Entity<'a> {
  fn get_drawable(&'a self) -> *const dyn Drawable<'a>;
}
