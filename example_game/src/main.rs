use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

extern crate bowtie;
extern crate futures;
extern crate rand;

use rand::Rng;

use bowtie::{
  glfw,
  premade_components::{
    CollisionComponent, GravityComponent, KeyboardMoveComponent,
  },
  BowTie, Color, Direction, Entity, Message, Rectangle, Sprite,
  StandardComponent, StandardEntity, Texture, TextureOptions, WindowConfig,
  WindowMode, COLORS,
};

fn main() {
  let mut collision = CollisionComponent::new();
  let mut gravity = GravityComponent::new(0.002);
  let mut keyboard_move = KeyboardMoveComponent::new(0.01);

  let collision_comp = collision.component();
  let gravity_comp = gravity.component();
  let keyboard_move_comp = keyboard_move.component();

  let mut bowtie = BowTie::new();
  bowtie.create_window(WindowConfig {
    width: 1400,
    height: 900,
    name: String::from("rust game engine"),
    mode: WindowMode::Windowed,
    monitor_idx: 1,
  });

  bowtie.prep_for_render();

  let witch_new_texture =
    Texture::new("witch_walk_1", TextureOptions::default());

  let bat_texture = Texture::new("bat_fly_1", TextureOptions::default());

  let backdrop_texture = Texture::new("backdrop", TextureOptions::default());

  let running_frames = Rc::new(Vec::from_iter(
    vec![
      "witch_walk_1",
      "witch_walk_2",
      "witch_walk_3",
      "witch_walk_4",
      "witch_walk_5",
      "witch_walk_6",
      "witch_walk_7",
      "witch_walk_8",
    ]
    .iter()
    .map(|image_file_name| {
      let original_texture =
        Texture::new(image_file_name, TextureOptions::default());
      let ref_copy = Texture::from(&original_texture);
      return [original_texture, ref_copy];
    })
    .flatten(),
  ));

  let bat_fly_1 = Texture::new("bat_fly_1", TextureOptions::default());
  let bat_fly_2 = Texture::new("bat_fly_2", TextureOptions::default());

  let flying_frames = Rc::new(vec![
    Texture::from(&bat_fly_1),
    Texture::from(&bat_fly_1),
    Texture::from(&bat_fly_1),
    Texture::from(&bat_fly_1),
    Texture::from(&bat_fly_1),
    Texture::from(&bat_fly_1),
    Texture::from(&bat_fly_1),
    Texture::from(&bat_fly_1),
    Texture::from(&bat_fly_2),
    Texture::from(&bat_fly_2),
    Texture::from(&bat_fly_2),
    Texture::from(&bat_fly_2),
    Texture::from(&bat_fly_2),
    Texture::from(&bat_fly_2),
    Texture::from(&bat_fly_2),
    Texture::from(&bat_fly_2),
    Texture::from(&bat_fly_2),
    Texture::from(&bat_fly_2),
    Texture::from(&bat_fly_2),
    Texture::from(&bat_fly_2),
    Texture::from(&bat_fly_2),
    Texture::from(&bat_fly_2),
  ]);

  let backdrop = Rc::new(RefCell::new(StandardEntity::new(
    Sprite::new(
      Rectangle::new(-1.0, 1.0, 2.0, 2.0, COLORS::White.into()),
      Texture::from(&backdrop_texture),
    ),
    0.0,
    Rc::new(vec![]),
  )));

  let floor = Rc::new(RefCell::new(StandardEntity::new(
    Sprite::new(
      Rectangle::new(-1.0, -0.5, 2.0, 0.02, Color::new(1.0, 1.0, 1.0, 0.1)),
      Texture::none(),
    ),
    0.0,
    Rc::new(vec![]),
  )));

  floor.borrow_mut().load_component(collision_comp.to_owned());

  let playable_character = Rc::new(RefCell::new(StandardEntity::new(
    Sprite::new(
      Rectangle::new(0.5, 0.0, 0.06, 0.15, COLORS::White.into()),
      Texture::from(&witch_new_texture),
    ),
    0.0,
    running_frames,
  )));

  {
    let mut playable_character_ref = playable_character.borrow_mut();
    playable_character_ref.load_component(collision_comp.to_owned());
    playable_character_ref.load_component(keyboard_move_comp.to_owned());
    playable_character_ref.load_component(gravity_comp.to_owned());
  }

  let circle_move = StandardComponent::new(
    Rc::new(|entity, _| {
      let current_direction = entity.get_direction();
      let current_speed = entity.get_speed();
      let next_direction = if current_speed >= 0.008 {
        match current_direction {
          Direction::UpLeft => Direction::UpRight,
          Direction::UpRight => Direction::DownRight,
          Direction::DownRight => Direction::DownLeft,
          Direction::DownLeft => Direction::UpLeft,
          Direction::Stationary => match rand::thread_rng().gen_range(1..4) {
            1 => Direction::UpLeft,
            2 => Direction::UpRight,
            3 => Direction::DownLeft,
            4 => Direction::DownRight,
            _ => Direction::DownRight,
          },
          _ => {
            panic!("CIrcle Move does not support external movement.")
          }
        }
      } else {
        current_direction
      };
      entity.set_direction(next_direction);
      entity.move_in_direction(next_direction, entity.get_speed() * rand::thread_rng().gen_range(0.85..1.8),
          );

      if current_direction != next_direction {
        entity.set_speed(0.001)
      } else {
        entity.set_speed(current_speed + 0.0005);
      }
    }),
    "circle_move",
    HashMap::new(),
  );

  bowtie.load_entity(Rc::clone(&backdrop));

  let mut bats: Vec<Rc<RefCell<StandardEntity>>> = vec![];

  for _ in 0..600 {
    let bat = Rc::new(RefCell::new(StandardEntity::new(
      Sprite::new(
        Rectangle::new(
          // (((i % 20) as f32) - 9.8) / 10.0,
          rand::thread_rng().gen_range(-0.94..0.95),
          rand::thread_rng().gen_range(-0.4..0.95),
          0.07,
          0.11,
          Color::new(
          rand::thread_rng().gen_range(0.0..1.0),
          rand::thread_rng().gen_range(0.0..1.0),
          rand::thread_rng().gen_range(0.0..1.0),
          rand::thread_rng().gen_range(0.7..0.8),
          )
        ),
        Texture::from(&bat_texture),
      ),
      0.0,
      Rc::clone(&flying_frames),
    )));
    bat.borrow_mut().load_component(circle_move.to_owned());
    bats.push(bat);
  }

  for bat in &bats {
    bowtie.load_entity(Rc::clone(&bat));
  }

  bowtie.load_entity(Rc::clone(&playable_character));
  bowtie.load_entity(Rc::clone(&floor));

  while !bowtie.should_close() {
    bowtie.tick();
    let events = bowtie.flush_events();

    playable_character.borrow_mut().animate();
    for bat in &bats {
      bat.borrow_mut().animate();
    }

    for event in events {
      keyboard_move.listen_for_event(Rc::clone(&playable_character), &event);

      match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, _, _) => {
          bowtie.set_should_close(true);
        }
        glfw::WindowEvent::Key(glfw::Key::E, _, glfw::Action::Press, _) => {
          playable_character
            .borrow_mut()
            .sprite
            .set_texture(witch_new_texture.clone())
        }

        glfw::WindowEvent::Key(glfw::Key::R, _, glfw::Action::Press, _) => {
          playable_character.borrow_mut().set_x(0.0);
          playable_character.borrow_mut().set_y(0.0);
        }
        glfw::WindowEvent::Key(glfw::Key::O, _, glfw::Action::Press, _) => {
          for _ in 0..100 {
            let rand_entity = Rc::new(RefCell::new(StandardEntity::new(
              Sprite::new(
                Rectangle::new(
                  rand::thread_rng().gen_range(-1.0..1.0) - 0.1,
                  rand::thread_rng().gen_range(-1.0..1.0) + 0.3,
                  0.06,
                  0.08,
                  Color::new(1.0, 0.0, 0.0, 0.3),
                ),
                Texture::from(&witch_new_texture),
              ),
              2.0,
              Rc::new(vec![]),
            )));
            //rand_entity.load_components(rand_move1.component());
            rand_entity
              .borrow_mut()
              .load_component(gravity_comp.to_owned());
            let rand_entity_id = bowtie.load_entity(rand_entity);
          }
          println!("Handling {} entities", bowtie.get_entity_count());
        }
        _ => {}
      }
    }
  }
}
