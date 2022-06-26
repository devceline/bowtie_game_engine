use crate::sprites::drawable::Drawable;

pub trait Entity<'a> {
  fn get_drawable(&'a self) -> &'a dyn Drawable<'a>;
}
