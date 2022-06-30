use crate::sprites::drawable::Drawable;
use std::collections::HashMap;

pub trait Entity<'a> {
  fn get_drawable(&'a self) -> &'a dyn Drawable<'a>;
  fn get_x(&self) -> f32;
  fn get_y(&self) -> f32;
  fn set_x(&mut self, x: f32) -> bool;
  fn set_y(&mut self, y: f32) -> bool;
  fn get_height(&self) -> f32;
  fn get_width(&self) -> f32;
  fn get_components(&mut self) -> &Vec<*mut dyn Component<'a>>;
  fn load_components(&mut self, component: *mut dyn Component<'a>);
  fn recieve_message(&mut self, message: Message);
}

pub trait Component<'a> {
  fn get_name(&self) -> &str;
  unsafe fn act(
    &mut self,
    entities: &Vec<*mut dyn Entity<'a>>,
    entity: *mut dyn Entity<'a>,
  ) -> Option<Message>;
}

pub struct Message {
  message_type: String,
  values: HashMap<String, f32>,
}

impl Message {
  pub fn new(message_type: String, values: HashMap<String, f32>) -> Message {
    Message {
      message_type,
      values,
    }
  }

  pub fn get_message_type(&self) -> String {
    self.message_type.to_owned()
  }

  pub fn get_values(&self) -> HashMap<String, f32> {
    self.values.to_owned()
  }
}
