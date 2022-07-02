use crate::sprites::drawable::Drawable;
use std::collections::HashMap;

/// Entity trait
/// This is the basis for any object that can be acted upon within the engine
/// 
/// Should have mutable position and should store its own components
/// Should also have a function that recieves messages so the components
/// have an effect.
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

  /// Implementing this will usually involve pattern matching or if statements
  /// to act depending on the type of message.
  fn recieve_message(&mut self, message: Message);
}

/// Component Trait
/// This is the basis for creating a component or system that acts within 
/// the engine
///
/// E.g: A collision component
pub trait Component<'a> {
  fn get_name(&self) -> &str;

  /// Act function recieves information about current entities and returns a 
  /// message with a type and a HashMap of values. This is to say, entities can
  /// interact with this information differently.
  ///
  /// E.g: An entity may choose to ignore a report of collision.
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
