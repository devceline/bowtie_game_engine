use crate::sprites::drawable::Drawable;

pub trait Entity<'a> {
  fn get_drawable(&'a self) -> &'a dyn Drawable<'a>;
  fn get_x(&self) -> f32;
  fn get_y(&self) -> f32;
  fn get_height(&self) -> f32;
  fn get_components(&mut self) -> &Vec<*mut dyn Component<'a>>;
  fn load_components(&mut self, component: *mut dyn Component<'a>);
  fn get_width(&self) -> f32;
}

pub trait Component<'a> {
  unsafe fn act(&mut self, entities: &Vec<*mut dyn Entity<'a>>, entity: *mut dyn Entity<'a>);
}
