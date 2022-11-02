use std::{marker, rc::Rc, sync::Arc, collections::HashMap};

use bowtie::{Direction, StandardComponent, StandardEntity};
extern crate rand;
use rand::Rng;

#[derive(Clone)]
pub struct RandMove<'a> {
  direction: Direction,
  magnitude: f32,
  _marker: marker::PhantomData<&'a i32>,
}

impl<'a> RandMove<'a> {
  pub fn new() -> RandMove<'a> {
    let direction = Direction::from(rand::thread_rng().gen_range(0..8));
    let magnitude = rand::thread_rng().gen_range(0.0..0.05);

    RandMove {
      direction,
      magnitude,
      _marker: marker::PhantomData,
    }
  }

  pub fn component(&'a self) -> StandardComponent<'a> {
    let direction = Direction::from(rand::thread_rng().gen_range(0..8));
    let magnitude = rand::thread_rng().gen_range(0.0..0.03);
    StandardComponent::new(Arc::new(move |entity, _store| {
      entity.move_in_direction(direction, magnitude);
    }), "rand_move", HashMap::new())
    .to_owned()
  }
}
