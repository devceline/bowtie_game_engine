use std::{
  collections::HashMap,
  marker::PhantomData,
  sync::{Arc, Mutex},
};

use crate::{
  bowtie::entity::{Component, Entity, Message},
  general::{direction::Direction, value::Value},
  math::general::absolute_value_f32,
  StandardComponent, StandardEntity,
};

pub type CollidingObjects<'d> =
  HashMap<*mut StandardEntity<'d>, Vec<(*mut StandardEntity<'d>, Direction)>>;

pub type CollidingObjectsArc<'d> = Arc<Mutex<CollidingObjects<'d>>>;

/// Collision Component
///
/// Sends a message reporting the current direction of collision
///
/// If an entity has not collided, the `Direction` will be `Direction::Stationary`
pub struct CollisionComponent<'d> {
  colliding_objects: CollidingObjectsArc<'d>,
  _marker: PhantomData<&'d i32>,
}

impl<'d> CollisionComponent<'d> {
  pub fn new() -> CollisionComponent<'d> {
    CollisionComponent {
      colliding_objects: Arc::new(Mutex::new(HashMap::new())),
      _marker: PhantomData,
    }
  }

  fn get_collision_direction(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    other_x: f32,
    other_y: f32,
    other_height: f32,
    other_width: f32,
  ) -> Direction {
    let mut direction = Direction::Stationary;

    let x_range = absolute_value_f32(x - other_x);
    let y_range = absolute_value_f32(y - other_y);

    let in_x_range = x_range <= width || x_range <= other_width;
    let in_y_range = y_range <= height;

    if !(in_x_range && in_y_range) {
      return direction;
    }

    let left_position = x;
    let right_position = x + width;

    let other_left_position = other_x;
    let other_right_position = other_x + other_width;

    let top_position = y;
    let bottom_position = y - height;

    let other_top_position = other_y;
    let other_bottom_position = other_y - other_height;

    let down_collision = bottom_position <= other_top_position
      && bottom_position >= other_bottom_position;

    let up_collision = top_position >= other_bottom_position
      && top_position <= other_top_position;

    let right_collision = right_position >= other_left_position
      && right_position <= other_right_position;

    let left_collision = left_position > other_left_position
      && left_position < other_right_position;

    if right_collision {
      direction = direction.add_direction(Direction::Right);
    }
    if left_collision {
      direction = direction.add_direction(Direction::Left);
    }

    if down_collision {
      direction = direction.add_direction(Direction::Down);
    }
    if up_collision {
      direction = direction.add_direction(Direction::Up);
    }

    return direction;
  }

  pub fn get_is_collided(&self, entity_ref: *mut StandardEntity<'d>) -> bool {
    let is_collided =
      match self.colliding_objects.lock().unwrap().get(&entity_ref) {
        Some(collision_vec) => collision_vec.len() > 0,
        None => false,
      };

    return is_collided;
  }

  pub fn get_entity_collision_direction(
    colliding_objects: &CollidingObjects<'d>,
    entity_ref: *mut StandardEntity<'d>,
  ) -> Direction {
    let mut collision_direction = Direction::Stationary;

    match colliding_objects.get(&entity_ref) {
      Some(collision_vec) => {
        for (_, collision_dir) in collision_vec {
          collision_direction =
            collision_direction.add_direction(collision_dir.to_owned());
        }
      }
      None => {}
    }

    return collision_direction;
  }

  unsafe fn calculate_collision_direction(
    entity: *const StandardEntity<'d>,
    other_entity: *const StandardEntity<'d>,
  ) -> Direction {
    let entity_unwrapped = entity.as_ref().unwrap();
    let other_entity_unwrapped = other_entity.as_ref().unwrap();

    let other_x = other_entity_unwrapped.get_x();
    let other_y = other_entity_unwrapped.get_y();
    let other_height = other_entity_unwrapped.get_height();
    let other_width = other_entity_unwrapped.get_width();

    let x = entity_unwrapped.get_x();
    let y = entity_unwrapped.get_y();
    let height = entity_unwrapped.get_height();
    let width = entity_unwrapped.get_width();

    let collision_direction = CollisionComponent::get_collision_direction(
      x,
      y,
      width,
      height,
      other_x,
      other_y,
      other_height,
      other_width,
    );

    collision_direction
  }

  unsafe fn get_final_collission_direction(
    colliding_objects: &CollidingObjectsArc<'d>,
    entity: *mut StandardEntity<'d>,
  ) -> Direction {
    let mut objects = colliding_objects.lock().unwrap();

    let keys = objects
      .keys()
      .map(|k| k.to_owned())
      .collect::<Vec<*mut StandardEntity<'d>>>();

    let current_vec = objects.entry(entity).or_insert(Vec::new());
    for other_entity in keys {
      if other_entity.as_ref().unwrap() as *const _
        == entity.as_ref().unwrap() as *const _
      {
        continue;
      }

      let collision_direction =
        CollisionComponent::calculate_collision_direction(entity, other_entity);

      let other_entity_position = (*current_vec)
        .iter()
        .position(|(ex_collided_entity, _)| *ex_collided_entity == other_entity);

      if collision_direction != Direction::Stationary {
        match other_entity_position {
          Some(_) => {}
          None => {
            (*current_vec).push((other_entity.to_owned(), collision_direction));
          }
        }
      } else {
        match other_entity_position {
          Some(pos) => {
            (*current_vec).remove(pos);
          }
          None => {}
        }
      }
    }

    CollisionComponent::get_entity_collision_direction(&objects, entity)

  }

  pub fn get_name() -> String {
    String::from("collision")
  }

  pub fn component(&'d mut self) -> StandardComponent<'d> {
    StandardComponent::new(
      Arc::new(|entity, store| unsafe {
        let new_collision_direction = CollisionComponent::get_final_collission_direction(
          &self.colliding_objects,
          entity,
        );
        let entity_ptr: *const StandardEntity<'d> = entity;

        let direction_value = Value::Number(new_collision_direction.into());

        let mut locked_store = store.lock().unwrap();
        let dir = locked_store.entry(format!("{:?}", entity_ptr)).or_insert(direction_value.to_owned());
        *dir = direction_value.to_owned();

        // println!("Setting collision direction to {:?}", new_collision_direction);

        entity.set_collision_direction(new_collision_direction);
      }),
      CollisionComponent::get_name().as_str(),
      HashMap::new(),
    )
  }
}
