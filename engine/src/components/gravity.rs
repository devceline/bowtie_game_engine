use std::cell::RefCell;
use std::rc::Rc;

use std::{collections::HashMap, marker::PhantomData};

use crate::{
  bowtie::component::ComponentStore,
  bowtie::entity::{Entity, StandardEntity},
  general::value::Value,
  Direction, StandardComponent,
};

/// Gravity Component
///
/// Sends a message about y position updates in `f32`
#[derive(Clone)]
pub struct GravityComponent {
  speed: f32,
  acceleration: f32,
  _marker: PhantomData<f32>,
  terminal_velocity: f32,
}

impl GravityComponent {
  pub fn new(speed: f32) -> GravityComponent {
    GravityComponent {
      speed: speed / 100.0,
      acceleration: speed * 0.2,
      terminal_velocity: speed * 1000.0,
      _marker: PhantomData,
    }
  }

  pub fn apply_gravity(
    &self,
    entity: Box<&mut StandardEntity>,
    store: Rc<RefCell<ComponentStore>>,
  ) {
    let mut borrowed_store = store.borrow_mut();
    let falling_objects = borrowed_store
      .entry(String::from("falling_objects"))
      .or_insert(Value::Object(HashMap::new()));

    let self_box = Box::from(self);

    match falling_objects {
      Value::Object(objects) => {
        // let entity_ptr: *mut StandardEntity = entity.as_mut();
        let entity_ptr: *mut StandardEntity = *entity as *mut StandardEntity;
        let entity_ptr_str = format!("{:?}", entity_ptr);

        let object_info = objects.entry(entity_ptr_str).or_insert(
          Value::Vec2f32((self.speed.to_owned(), entity.get_y().to_owned())),
        );

        match object_info {
          Value::Vec2f32((speed, y_pos)) => {
            if *speed < self_box.terminal_velocity {
              *speed += self_box.acceleration;
            }

            let entity_y = entity.get_y();

            // If they have stopped falling, reset their speed.
            if entity_y == *y_pos {
              *speed = self_box.speed;
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
  }

  pub fn get_name() -> String {
    String::from("gravity")
  }

  pub fn component(&mut self) -> StandardComponent {
    let self_terminal_velocity = self.terminal_velocity.to_owned();
    let self_acceleration = self.acceleration.to_owned();
    let self_speed = self.speed.to_owned();

    StandardComponent::new(
      Rc::new(move |entity, store| {
        let mut borrowed_store = store.borrow_mut();
        let falling_objects = borrowed_store
          .entry(String::from("falling_objects"))
          .or_insert(Value::Object(HashMap::new()));

        match falling_objects {
          Value::Object(objects) => {
            // let entity_ptr: *mut StandardEntity = entity.as_mut();
            let entity_ptr: *mut StandardEntity = entity;
            let entity_ptr_str = format!("{:?}", entity_ptr);

            let object_info = objects.entry(entity_ptr_str).or_insert(
              Value::Vec2f32((self_speed.to_owned(), entity.get_y().to_owned())),
            );

            match object_info {
              Value::Vec2f32((speed, y_pos)) => {
                if *speed < self_terminal_velocity {
                  *speed += self_acceleration;
                }

                let entity_y = entity.get_y();

                // If they have stopped falling, reset their speed.
                if entity_y == *y_pos {
                  *speed = self_speed;
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
      HashMap::from([(
        String::from("falling_objects"),
        Value::Object(HashMap::new()),
      )]),
    )
  }
}
