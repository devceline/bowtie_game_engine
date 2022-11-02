extern crate bowtie;
extern crate rand;
use rand::Rng;

use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use bowtie::{
  glfw, init_debug_callback, math,
  premade_components::{
    CollisionComponent, GravityComponent, KeyboardMoveComponent,
  },
  BowTie, Color, Direction, Entity, Message, Rectangle, Sprite,
  StandardComponent, StandardEntity, Texture, TextureOptions, Value,
  WindowConfig, WindowMode, COLORS,
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

  bowtie.prep_for_render();

  bowtie.load_entity(StandardEntity::new(Sprite::new(Rectangle::new(-1.0, 1.0, 2.0, 2.0, COLORS::White.into()), Texture::new("space", TextureOptions::default()))));

  let mut player_rect = StandardEntity::new(Sprite::new(
    Rectangle::new(-0.6, 0.0, 0.05, 0.6, Color::new(1.0, 1.0, 1.0, 0.3)),
    Texture::none(),
  ));

  let mut enemy_rect = StandardEntity::new(Sprite::new(
    Rectangle::new(0.5, 0.4, 0.05, 0.6, Color::new(1.0, 1.0, 1.0, 0.3)),
    Texture::none(),
  ));

  let ball_ptr = bowtie.load_entity(StandardEntity::new(Sprite::new(
    Rectangle::new(0.0, -0.2, 0.1, 0.1, COLORS::Violet.into()),
    Texture::new("whitecircle", TextureOptions::default()),
  )));

  let ball = unsafe { ball_ptr.as_mut().unwrap() };


  let line_thickness = 0.01;
  let line_vectors = [
    (-1.0, 1.0, 2.0, line_thickness),
    (-1.0, 1.0, line_thickness, 2.0),
    (1.0 - line_thickness, 1.0, line_thickness, 2.0),
    (-1.0, -1.0 + line_thickness, 2.0, line_thickness),
  ];

  for line in line_vectors {
    let (x, y, w, h) = line;
    let created_line = bowtie.load_entity(StandardEntity::new(Sprite::new(
      Rectangle::new(x, y, w, h, COLORS::Black.into()),
      Texture::none(),
    )));
    unsafe {
      created_line
        .as_mut()
        .unwrap()
        .load_components(collision_comp.to_owned());
    }
  }
  // ball.move_in_direction(Direction::Right, 0.7);



  let move_till_collide = StandardComponent::new(
    Arc::new(|entity, store| {
      let mut new_color: (f32, f32, f32) = (0.5, 0.5, 0.5);
      let mut collided = false;
      let mut store_locked = store.lock().unwrap();
      let current_direction_val = store_locked
        .entry(String::from("direction"))
        .or_insert(Value::Number(Direction::UpRight.into()));

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

              if col_x + col_x != 0.0 {
                let r: f32 = rand::thread_rng().gen_range(0.0..0.5);
                let g: f32 = rand::thread_rng().gen_range(0.0..0.5);
                let b: f32 = rand::thread_rng().gen_range(0.0..0.5);

                new_color = (r, g, b);
                collided = true;
              }

              cur_direction = Direction::from_vector((new_x, new_y));
              *current_direction_val = Value::Number(cur_direction.into());
            }
          }
        }

        if collided {
          let (r, g, b) = new_color;
          entity.set_color(Color::new(r, g, b, 1.0));
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

  let ball_ptr: *const StandardEntity = ball;

  let follow_ball_comp = StandardComponent::new(
    Arc::new(move |entity, store| {
      let key = String::from("direction");
      let mut store_locked = store.lock().unwrap();
      let current_direction_val = store_locked.get_mut(&key).unwrap();

      let ball_ref = unsafe { ball_ptr.as_ref().unwrap() };

      if let Value::Number(direction_num) = current_direction_val {
        entity
          .move_in_direction(Direction::from(direction_num.to_owned()), 0.02);

        let entity_midpoint = entity.get_y() - (entity.get_height() / 2.0);

        let y = ball_ref.get_y() - (ball_ref.get_height() / 2.0);

        // println!("entity_m {entity_midpoint}, ball_m {y}");

        if entity_midpoint < y && entity.get_y() < 0.9 {
          *current_direction_val = Value::Number(Direction::Up.into());
        } else if entity_midpoint > y {
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
  ball.set_color(COLORS::Black.into());
  ball.load_components(collision_comp.to_owned());
  ball.load_components(move_till_collide);

  bowtie.load_entity(player_rect);
  bowtie.load_entity(enemy_rect);

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
