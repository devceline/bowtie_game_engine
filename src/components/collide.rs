use std::{collections::HashMap, marker::PhantomData};

use crate::god_object::entity::{Component, Entity};

pub struct CollisionComponent<'d> {
  colliding_objects: HashMap<*mut dyn Entity<'d>, Vec<*mut dyn Entity<'d>>>,
  _marker: PhantomData<&'d i32>,
}

impl<'d> CollisionComponent<'d> {
  pub fn new() -> CollisionComponent<'d> {
    CollisionComponent {
      colliding_objects: HashMap::new(),
      _marker: PhantomData,
    }
  }

  pub fn get_is_collided(&self, entity_ref: *mut dyn Entity<'d>) -> bool {
    let is_collided = match self.colliding_objects.get(&entity_ref) {
      Some(collision_vec) => collision_vec.len() > 0,
      None => false,
    };

    return is_collided;
  }
}

impl<'d> Component<'d> for CollisionComponent<'d> {
  unsafe fn act(
    &mut self,
    entities: &Vec<*mut dyn Entity<'d>>,
    entity: *mut dyn Entity<'d>,
  ) {
    for other_entity in entities {
      if other_entity.as_ref().unwrap() as *const _
        == entity.as_ref().unwrap() as *const _
      {
        continue;
      }

      let current_vec =
        self.colliding_objects.entry(entity).or_insert(Vec::new());

      let other_x = other_entity.as_ref().unwrap().get_x();
      let other_y = other_entity.as_ref().unwrap().get_y();
      let other_height = other_entity.as_ref().unwrap().get_height();
      let other_width = other_entity.as_ref().unwrap().get_width();

      let x = entity.as_ref().unwrap().get_x();
      let y = entity.as_ref().unwrap().get_y();
      let height = entity.as_ref().unwrap().get_height();
      let width = entity.as_ref().unwrap().get_width();

      let x_collision = (x + width) >= other_x && x <= (other_x + other_width);
      let y_collision = y + height >= other_y && y <= (other_y + other_height);

      let other_entity_position = (*current_vec)
        .iter()
        .position(|ex_collided_entity| ex_collided_entity == other_entity);

      if x_collision && y_collision {
        match other_entity_position {
          Some(_) => {}
          None => {
            (*current_vec).push(other_entity.to_owned());
          }
        }
      } else {
          match other_entity_position {
            Some(pos) => {
              (*current_vec).remove(pos);
            }
            None => {
            }
          }
      }
    }
  }
}
