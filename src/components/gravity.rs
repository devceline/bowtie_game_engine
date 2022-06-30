use std::collections::HashMap;

use crate::god_object::entity::{Component, Entity, Message};

pub struct GravityComponent<'s> {
  speed: f32,
  acceleration: f32,
  terminal_velocity: f32,
  // Although this is not scientifcally accurate, I CBA implement weight.
  falling_objects: HashMap<*mut dyn Entity<'s>, (f32, f32)>,
}

impl<'s> GravityComponent<'s> {
  pub fn new(speed: f32) -> GravityComponent<'s> {
    GravityComponent {
      speed: speed / 100.0,
      acceleration: speed * 0.2,
      terminal_velocity: speed * 1000.0,
      falling_objects: HashMap::new(),
    }
  }

  pub fn get_message_name() -> String {
    "gravity_pull".to_string()
  }
}

impl<'s> Component<'s> for GravityComponent<'s> {
  fn get_name(&self) -> &str {
    "gravity"
  }

  unsafe fn act(
    &mut self,
    _entities: &Vec<*mut dyn Entity<'s>>,
    entity: *mut dyn Entity<'s>,
  ) -> Option<Message> {
    let entity_ref = entity.as_ref().unwrap();

    let (speed, y_pos) = self
      .falling_objects
      .entry(entity)
      .or_insert((self.speed, entity_ref.get_y()));

    if *speed < self.terminal_velocity {
      *speed += self.acceleration;
    }

    let entity_y = entity_ref.get_y();

    // If they have stopped falling, reset their speed.
    if entity_y == *y_pos {
      *speed = self.speed;
    } else {
      *y_pos = entity_y;
    }

    Some(Message::new(
      GravityComponent::get_message_name(),
      HashMap::from([(String::from("speed"), *speed)]),
    ))
  }
}
