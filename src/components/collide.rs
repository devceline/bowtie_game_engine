use std::{collections::HashMap, marker::PhantomData};

use crate::god_object::entity::{Component, Entity};

pub struct CollisionComponent<'d> {
  colliding_objects: HashMap<i32, Vec<i32>>,
  _marker: PhantomData<&'d i32>
}

impl<'d> CollisionComponent<'d> {
  pub fn new() -> CollisionComponent<'d> {
    CollisionComponent { colliding_objects: HashMap::new(),_marker: PhantomData  }
  }
}

impl<'d> Component<'d> for CollisionComponent<'d> {

  
  unsafe fn act(&mut self, entities: &Vec<*mut dyn Entity<'d>>, entity: *mut dyn Entity<'d>) {
    let entity_id = 
      entities
      .iter()
      .position(|entity_ptr| {
        let other_entity_ptr_thin = entity_ptr.as_ref().unwrap() as *const _;
        let entity_ptr_thin = entity.as_ref().unwrap() as *const _;
        return other_entity_ptr_thin == entity_ptr_thin;
      })
      .expect("Entity not registered in entities") as i32;

    for other_entity in entities {
      let other_entity_id = 
        entities
        .iter()
        .position(|entity_ptr| entity_ptr == other_entity)
        .expect("Other Entity not registered in entities") as i32;

        if other_entity_id == entity_id {
          continue;
        }

        let other_x = other_entity.as_ref().unwrap().get_x();
        let other_y = other_entity.as_ref().unwrap().get_y();
        let other_height = other_entity.as_ref().unwrap().get_height();
        let other_width = other_entity.as_ref().unwrap().get_width();

        let x = entity.as_ref().unwrap().get_x();
        let y = entity.as_ref().unwrap().get_y();
        let height = entity.as_ref().unwrap().get_height();
        let width = entity.as_ref().unwrap().get_width();

        let x_collision = (x + width) >= other_x && x <= (other_x + other_width);
        let y_collision = y >= other_y && y <= (other_y + other_height);

        let current_vec = self
          .colliding_objects
          .entry(entity_id)
          .or_insert(Vec::new());
        
          println!("x_Collision {x_collision}, y_Collision {y_collision} \n");
          println!("Other: {other_x}, {other_y}");
          println!("Self: {x}, {y}");

        if x_collision && y_collision {
          (*current_vec).push(other_entity_id);
        }
        else {
          if current_vec.len() > other_entity_id as usize {
            (*current_vec).remove(other_entity_id as usize);
          }
        }

    }
  }
}
