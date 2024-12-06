use std::{marker, rc::Rc, sync::Arc, collections::HashMap};

use bowtie::{Direction, StandardComponent, StandardEntity};
extern crate rand;
use rand::Rng;

#[derive(Clone)]
pub struct RandMove {
}

impl RandMove {
  pub fn new() -> RandMove {
    RandMove {}
  }

  pub fn component(&self) -> StandardComponent {
    let direction = Direction::from(rand::thread_rng().gen_range(0..8));
    let magnitude = rand::thread_rng().gen_range(0.0..0.03);
    StandardComponent::new(Rc::new(move |entity, _store| {
      entity.move_in_direction(direction, magnitude);
    }), "rand_move", HashMap::new())
    .to_owned()
  }
}
