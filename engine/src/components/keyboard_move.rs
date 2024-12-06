use std::{
  collections::HashMap,
  marker::PhantomData,
  rc::Rc,
  cell::RefCell,
  sync::{Arc, Mutex},
};

use crate::{
  general::value::Value, premade_components::CollisionComponent, Direction,
  StandardComponent, StandardEntity,
};

#[derive(Clone)]
pub struct KeyboardMoveComponent {
  next_direction: Direction,
  top_speed: f32,
  _marker: PhantomData<f32>,
}

impl KeyboardMoveComponent {
  pub fn new(
    top_speed: f32,
  ) -> KeyboardMoveComponent {
    KeyboardMoveComponent {
      next_direction: Direction::Stationary,
      top_speed,
      _marker: PhantomData,
    }
  }

  pub fn get_name() -> String {
    String::from("keyboard_move")
  }

  pub fn move_component(
    entity: &mut StandardEntity,
    top_speed: f32
  ) {
    let speed_clone = entity.get_speed().clone();

    if entity.get_direction() == Direction::Stationary {
      return;
    }

    match entity.get_component(CollisionComponent::get_name().as_str()) {
      Some(comp) => {
        let collision_store = comp.get_store();
        let entity_ptr: *const StandardEntity = entity;
        let entity_id = format!("{:?}", entity_ptr);
        match collision_store.borrow().get(&entity_id) {
          None => {}
          Some(dir_num) => {
            match dir_num {
              Value::Number(num) => {
                let collision_direction = Direction::from(num.clone());
                entity.set_direction(entity.get_direction().subtract_direction(collision_direction))
              }
              _ => {}
            }
          }
        };
      }
      None => {}
    };

    entity.move_in_direction(entity.get_direction(), speed_clone);


    if speed_clone < top_speed {
      let acceleration = 0.2;
      entity.set_speed(entity.get_speed() + acceleration);
    }
  }

  pub fn listen_for_event(&mut self, event: &glfw::WindowEvent) {
    match event {
      glfw::WindowEvent::Key(glfw::Key::Right, _, glfw::Action::Press, _) => {
        self.next_direction = self.next_direction.add_direction(Direction::Right);
      }
      glfw::WindowEvent::Key(glfw::Key::Right, _, glfw::Action::Repeat, _) => {
        self.next_direction = self.next_direction.add_direction(Direction::Right);
      }
      glfw::WindowEvent::Key(glfw::Key::Right, _, glfw::Action::Release, _) => {
        self.next_direction = self.next_direction.subtract_direction(Direction::Right);
      }

      glfw::WindowEvent::Key(glfw::Key::Left, _, glfw::Action::Press, _) => {
        self.next_direction = self.next_direction.add_direction(Direction::Left);
      }
      glfw::WindowEvent::Key(glfw::Key::Left, _, glfw::Action::Repeat, _) => {
        self.next_direction = self.next_direction.add_direction(Direction::Left);
      }
      glfw::WindowEvent::Key(glfw::Key::Left, _, glfw::Action::Release, _) => {
        self.next_direction = self.next_direction.subtract_direction(Direction::Left);
      }

      glfw::WindowEvent::Key(glfw::Key::Up, _, glfw::Action::Press, _) => {
        self.next_direction = self.next_direction.add_direction(Direction::Up);
      }
      glfw::WindowEvent::Key(glfw::Key::Up, _, glfw::Action::Repeat, _) => {
        self.next_direction = self.next_direction.add_direction(Direction::Up);
      }
      glfw::WindowEvent::Key(glfw::Key::Up, _, glfw::Action::Release, _) => {
        self.next_direction = self.next_direction.subtract_direction(Direction::Up);
      }

      glfw::WindowEvent::Key(glfw::Key::Down, _, glfw::Action::Repeat, _) => {
        self.next_direction = self.next_direction.add_direction(Direction::Down);
      }
      glfw::WindowEvent::Key(glfw::Key::Down, _, glfw::Action::Press, _) => {
        self.next_direction = self.next_direction.add_direction(Direction::Down);
      }
      glfw::WindowEvent::Key(glfw::Key::Down, _, glfw::Action::Release, _) => {
        self.next_direction = self.next_direction.subtract_direction(Direction::Down);
      }
      _ => {}
    }
  }

  pub fn component(&self) -> StandardComponent {
    let self_top_speed = self.top_speed;
    StandardComponent::new(
      Rc::new(move |entity, _store| {
        KeyboardMoveComponent::move_component(
          entity,
          self_top_speed,
        );
      }),
      KeyboardMoveComponent::get_name().as_str(),
      HashMap::new(),
    )
  }
}
