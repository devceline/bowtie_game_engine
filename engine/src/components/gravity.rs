use std::{
  collections::HashMap,
  marker::PhantomData,
  sync::{Arc, Mutex},
};

use crate::{
  bowtie::entity::{Component, Entity, Message, StandardEntity},
  general::value::Value,
  Direction, StandardComponent,
};

/// Gravity Component
///
/// Sends a message about y position updates in `f32`
#[derive(Clone)]
pub struct GravityComponent<'s> {
  speed: f32,
  acceleration: f32,
  _marker: PhantomData<&'s f32>,
  terminal_velocity: f32,
}

impl<'s> GravityComponent<'s> {
  pub fn new(speed: f32) -> GravityComponent<'s> {
    GravityComponent {
      speed: speed / 100.0,
      acceleration: speed * 0.2,
      terminal_velocity: speed * 1000.0,
      _marker: PhantomData,
    }
  }

  pub fn get_name() -> String {
    String::from("gravity")
  }

  pub fn component(&'s mut self) -> StandardComponent<'s> {
    StandardComponent::new(
      Arc::new(|entity, store| {
        let mut locked_store = store.lock().unwrap();

        let falling_objects = locked_store
          .entry(String::from("falling_objects"))
          .or_insert(Value::Object(HashMap::new()));

        match falling_objects {
          Value::Object(objects) => {
            let entity_ptr: *mut StandardEntity<'s> = entity;
            let entity_ptr_str = format!("{:?}", entity_ptr);

            let object_info = objects
              .entry(entity_ptr_str)
              .or_insert(Value::Vec2f32((self.speed.to_owned(), entity.get_y().to_owned())));

            match object_info {
              Value::Vec2f32((speed, y_pos)) => {
                if *speed < self.terminal_velocity {
                  *speed += self.acceleration;
                }

                let entity_y = entity.get_y();

                // If they have stopped falling, reset their speed.
                if entity_y == *y_pos {
                  *speed = self.speed;
                } else {
                  *y_pos = entity_y;
                }

                entity.move_in_direction(Direction::Down, *speed);
              }
              _ => {}
            }

          }
          _ => {
            panic!("Falling objects should be hashmap")
          }
        }
      }),
      GravityComponent::get_name().as_str(),
      HashMap::from([
        (String::from("falling_objects"), Value::Object(HashMap::new()))
      ]),
    )
  }
}
