use std::{
  collections::HashMap,
  marker::PhantomData,
  sync::{Arc, Mutex},
};

use crate::{
  premade_components::CollisionComponent, Direction, StandardComponent,
  StandardEntity, general::value::Value,
};

#[derive(Clone)]
pub struct KeyboardMoveComponent<'s> {
  speed: Arc<Mutex<f32>>,
  acceleration: Arc<f32>,
  next_direction: Arc<Mutex<Direction>>,
  top_speed: Arc<f32>,
  _marker: PhantomData<&'s f32>,
}

impl<'s> KeyboardMoveComponent<'s> {

  pub fn new(
    speed: f32,
    acceleration: f32,
    top_speed: f32,
  ) -> KeyboardMoveComponent<'s> {
    KeyboardMoveComponent {
      speed: Arc::new(Mutex::new(speed)),
      acceleration: Arc::new(acceleration),
      next_direction: Arc::new(Mutex::new(Direction::Stationary)),
      top_speed: Arc::new(top_speed),
      _marker: PhantomData,
    }
  }

  pub fn get_name() -> String {
    String::from("keyboard_move")
  }

  pub fn move_component(
    entity: &mut StandardEntity<'s>,
    speed_arc: &Arc<Mutex<f32>>,
    acceleration_arc: &Arc<f32>,
    direction_arc: &Arc<Mutex<Direction>>,
    top_speed_arc: &Arc<f32>,
  ) {
    let mut speed = speed_arc.lock().unwrap();
    let mut direction = direction_arc.lock().unwrap().clone();
    let speed_clone = speed.clone();

    if direction == Direction::Stationary {
      return
    }

    match entity.get_component(CollisionComponent::get_name().as_str()) {
      Some(comp) => {
        let collision_store = comp.get_store().lock().unwrap();
        let entity_ptr: *const StandardEntity<'s> = entity;
        let entity_id = format!("{:?}", entity_ptr);
        match collision_store.get(&entity_id)  {
          None => {}
          Some(dir_num) => {
            match dir_num {
              Value::Number(num) => {
                // println!("{:?}", num);
                let collision_direction = Direction::from(num.clone());
                println!("Subtracting {:?} from {:?}", direction, collision_direction);
                direction = direction.subtract_direction(collision_direction);
              }
              _ => {}
            }
          }

        }
      }
      None => {}
    }

    entity.move_in_direction(direction, speed_clone);

    if speed_clone < top_speed_arc.as_ref().clone() {
      let acc = acceleration_arc.as_ref().clone();
      *speed = speed_clone + acc;
    }
  }

  pub fn listen_for_event(&self, event: &glfw::WindowEvent) {
    let mut direction = self.next_direction.lock().unwrap();
    match event {
      glfw::WindowEvent::Key(glfw::Key::Right, _, glfw::Action::Press, _) => {
        *direction = direction.add_direction(Direction::Right);
      }
      glfw::WindowEvent::Key(glfw::Key::Right, _, glfw::Action::Repeat, _) => {
        *direction = direction.add_direction(Direction::Right);
      }
      glfw::WindowEvent::Key(glfw::Key::Right, _, glfw::Action::Release, _) => {
        *direction = direction.subtract_direction(Direction::Right);
      }

      glfw::WindowEvent::Key(glfw::Key::Left, _, glfw::Action::Press, _) => {
        *direction = direction.add_direction(Direction::Left);
      }
      glfw::WindowEvent::Key(glfw::Key::Left, _, glfw::Action::Repeat, _) => {
        *direction = direction.add_direction(Direction::Left);
      }
      glfw::WindowEvent::Key(glfw::Key::Left, _, glfw::Action::Release, _) => {
        *direction = direction.subtract_direction(Direction::Left);
      }

      glfw::WindowEvent::Key(glfw::Key::Up, _, glfw::Action::Press, _) => {
        *direction = direction.add_direction(Direction::Up);
      }
      glfw::WindowEvent::Key(glfw::Key::Up, _, glfw::Action::Repeat, _) => {
        *direction = direction.add_direction(Direction::Up);
      }
      glfw::WindowEvent::Key(glfw::Key::Up, _, glfw::Action::Release, _) => {
        *direction = direction.subtract_direction(Direction::Up);
      }

      glfw::WindowEvent::Key(glfw::Key::Down, _, glfw::Action::Repeat, _) => {
        *direction = direction.add_direction(Direction::Down);
      }
      glfw::WindowEvent::Key(glfw::Key::Down, _, glfw::Action::Press, _) => {
        *direction = direction.add_direction(Direction::Down);
      }
      glfw::WindowEvent::Key(glfw::Key::Down, _, glfw::Action::Release, _) => {
        *direction = direction.subtract_direction(Direction::Down);
      }
      _ => {}
    }

    println!("Setting new direction to {:?}", direction);
  }

  pub fn component(&'s self) -> StandardComponent<'s> {
    StandardComponent::new(
      Arc::new(|entity, _store| {
        KeyboardMoveComponent::move_component(
          entity,
          &self.speed,
          &self.acceleration,
          &self.next_direction,
          &self.top_speed,
        );
      }),
      KeyboardMoveComponent::get_name().as_str(),
      HashMap::new(),
    )
  }
}
