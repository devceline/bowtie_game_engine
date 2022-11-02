extern crate bowtie;
extern crate futures;
extern crate rand;

mod components;

use components::rand_move::RandMove;
use rand::Rng;

use bowtie::{
  init_debug_callback, math,
  premade_components::{CollisionComponent, GravityComponent, KeyboardMoveComponent},
  BowTie, Direction, Entity, Message, Rectangle, Sprite, StandardComponent,
  StandardEntity, Texture, TextureOptions, COLORS, WindowMode, WindowConfig, glfw
};


fn main() {

  let mut collision = CollisionComponent::new();
  let rand_move1 = RandMove::new();
  let mut gravity = GravityComponent::new(0.002);
  let keyboard_move = KeyboardMoveComponent::new(0.02, 0.0, 0.4);

  let collision_comp = collision.component();
  let rand_comp = rand_move1.component();
  let gravity_comp = gravity.component();
  let keyboard_move_comp = keyboard_move.component();

  let mut bowtie = BowTie::new();
  bowtie.create_window(WindowConfig { width: 1000, height: 800, name: String::from("rust game engine"), mode: WindowMode::Windowed });

  let en_texture = Texture::new("witch", TextureOptions::default());

  let mut playable_character = StandardEntity::new(Sprite::new(Rectangle::new(0.0, 0.0, 0.2, 0.3, COLORS::White.into()), Texture::from(&en_texture)));
  playable_character.load_components(collision_comp.to_owned());
  playable_character.load_components(keyboard_move_comp.to_owned());

  let mut playable_character2 = StandardEntity::new(Sprite::new(Rectangle::new(0.5, 0.0, 0.2, 0.3, COLORS::White.into()), Texture::from(&en_texture)));
  playable_character2.load_components(collision_comp.to_owned());

  bowtie.load_entity(playable_character);

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
        glfw::WindowEvent::Key(glfw::Key::P, _, glfw::Action::Press, _) => {
          bowtie.load_entity(StandardEntity::new(
            Sprite::new(
              Rectangle::new(
                (rand::random::<f32>() % 1.0) - 0.5,
                (rand::random::<f32>() % 1.0) - 0.5,
                0.2,
                0.3,
                COLORS::White.into(),
              ),
              Texture::from(&en_texture),
            ),
          ));
        }
        glfw::WindowEvent::Key(glfw::Key::O, _, glfw::Action::Press, _) => {
          for _ in 0..100 {
            let mut rand_entity = StandardEntity::new(
              Sprite::new(
                Rectangle::new(
                  rand::thread_rng().gen_range(-1.0..1.0) - 0.1,
                  rand::thread_rng().gen_range(-1.0..1.0) + 0.3,
                  0.2,
                  0.3,
                  COLORS::Red.into(),
                ),
                Texture::from(&en_texture),
              ),
            );
            //rand_entity.load_components(rand_move1.component());
            rand_entity.load_components(gravity_comp.to_owned());
            bowtie.load_entity(rand_entity);
          }
          println!("Handling {} entities", bowtie.get_entity_count());
        }
        _ => {}
      }
    }
  }

}

