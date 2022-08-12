extern crate bowtie;

use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use bowtie::{
  glfw, init_debug_callback, math,
  premade_components::{
    CollisionComponent, GravityComponent, KeyboardMoveComponent,
  },
  BowTie, Direction, Entity, Message, Rectangle, Sprite, StandardComponent,
  StandardEntity, Texture, TextureOptions, Value, WindowConfig, WindowMode,
  COLORS,
};

fn main() {
  let mut collision = CollisionComponent::new();
  let keyboard_move = KeyboardMoveComponent::new(0.02, 0.0, 0.4);
  let keyboard_move_comp = keyboard_move.component();
  let collision_comp = collision.component();

  let mut bowtie = BowTie::new();
  bowtie.create_window(WindowConfig {
    width: 1000,
    height: 800,
    name: String::from("rust game engine"),
    mode: WindowMode::Windowed,
  });

  let mut player_rect = StandardEntity::new(
    Sprite::new(
      Rectangle::new(-0.6, 0.0, 0.1, 0.6, COLORS::Red.into()),
      Texture::none(),
    ),
    0.0,
  );

  let mut enemy_rect = StandardEntity::new(
    Sprite::new(
      Rectangle::new(0.5, 0.4, 0.1, 0.6, COLORS::Red.into()),
      Texture::none(),
    ),
    0.0,
  );

  let mut ball = StandardEntity::new(
    Sprite::new(
      Rectangle::new(0.0, -0.2, 0.1, 0.1, COLORS::Violet.into()),
      Texture::none(),
    ),
    0.0,
  );

  let move_till_collide = StandardComponent::new(
    Arc::new(|entity, store| {
      let key = String::from("direction");
      let mut store_locked = store.lock().unwrap();
      let current_direction_val = store_locked.get_mut(&key).unwrap();

      if let Value::Number(direction_num) = current_direction_val {
        let mut cur_direction = Direction::from(direction_num.clone());

        let comp = entity
          .get_component(CollisionComponent::get_name().as_str())
          .to_owned();

        if let Some(entity_collision_comp) = comp {
          let (cur_x, cur_y) = cur_direction.as_vector();

          let collision_store =
            entity_collision_comp.get_store().lock().unwrap();
          let entity_ptr: *const StandardEntity = entity;
          let entity_id = format!("{:?}", entity_ptr);

          if let Some(num) = collision_store.get(&entity_id) {
            if let Value::Number(dir_num) = num {
              let mut new_x: f32 = cur_x;
              let mut new_y: f32 = cur_y;
              let (col_x, col_y) = Direction::from(dir_num.clone()).as_vector();

              if col_x != 0.0 {
                new_x *= -1.0;
              }

              if col_y != 0.0 {
                new_y *= -1.0;
              }

              cur_direction = Direction::from_vector((new_x, new_y));
              *current_direction_val = Value::Number(cur_direction.into());
            }
          }
        }

        entity.move_in_direction(cur_direction, 0.02);
      }
    }),
    "move_till_collide",
    HashMap::from([(
      String::from("direction"),
      Value::Number(Direction::UpRight.into()),
    )]),
  );

  let follow_ball_comp = StandardComponent::new(
    Arc::new(|entity, store| {

      let key = String::from("direction");
      let mut store_locked = store.lock().unwrap();
      let current_direction_val = store_locked.get_mut(&key).unwrap();

      if let Value::Number(direction_num) = current_direction_val {
        entity.move_in_direction(Direction::from(direction_num.to_owned()), 0.05);

        if entity.get_y() - entity.get_height() < -0.9 {
          *current_direction_val = Value::Number(Direction::Up.into());
        } else if entity.get_y() > 0.9 {
          *current_direction_val = Value::Number(Direction::Down.into());
        } 

      }

    }),
    "follow_ball",
    HashMap::from([(
      String::from("direction"),
      Value::Number(Direction::Up.into()),
    )]),
  );

  player_rect.load_components(keyboard_move_comp);

  player_rect.load_components(collision_comp.to_owned());
  enemy_rect.load_components(collision_comp.to_owned());
  enemy_rect.load_components(follow_ball_comp.to_owned());
  ball.load_components(collision_comp.to_owned());
  ball.load_components(move_till_collide);

  bowtie.load_entity(player_rect);
  bowtie.load_entity(enemy_rect);
  bowtie.load_entity(ball);

  bowtie.prep_for_render();

  while !bowtie.should_close() {
    bowtie.tick();
    let events = bowtie.flush_events();
    for event in events {
      keyboard_move.listen_for_event(&event);
      match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, _, _) => {
          bowtie.set_should_close(true);
        }
        _ => {}
      }
    }
  }
}
