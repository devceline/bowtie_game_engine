extern crate futures;
extern crate gl;
extern crate glfw;
extern crate png;

mod components;
mod game_objects;
mod general;
mod gl_utils;
mod bowtie;
mod math;
mod rendering;
mod shapes;
mod sprites;

use components::collide::CollisionComponent;
use components::event::EventComponent;
use components::gravity::GravityComponent;
use game_objects::floor::Floor;
use glfw::Context;

use game_objects::playable_character::PlayableCharacter;
use general::color::COLORS;
use gl_utils::gl_error_reader;
use gl_utils::gl_texture::{Texture, TextureOptions};
use bowtie::entity::Entity;
use bowtie::bowtie::BowTie;
use shapes::rectangle::Rectangle;
use sprites::sprite::Sprite;

async fn handle_player_events<'a>(
  event: glfw::WindowEvent,
  character: &mut PlayableCharacter<'a>,
) {
  // println!("x: {}, y: {}, y+height: {}", character.get_x(), character.get_y(), character.get_y() + character.get_height());
  futures::join!(character.respond_to_event(&event));
}

fn window_setup(glfw: &mut glfw::Glfw, window: &mut glfw::Window) {
  window.make_current();

  gl::load_with(|s| glfw.get_proc_address_raw(s));

  // OpenGL 3.2
  glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
  glfw.window_hint(glfw::WindowHint::ContextVersionMinor(2));
  glfw.window_hint(glfw::WindowHint::OpenGlProfile(
    glfw::OpenGlProfileHint::Core,
  ));
  glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

  window.make_current();
  window.set_key_polling(true);
  window.set_sticky_keys(true);
}

fn main() {
  let mut glfw_instance = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  let (mut window, events) = glfw_instance
    .create_window(1000, 800, "rust game engine", glfw::WindowMode::Windowed)
    .expect("Failed to create window");
  window_setup(&mut glfw_instance, &mut window);

  gl_error_reader::init_debug_callback();
  let mut bowtie = BowTie::new();

  let mut collision_component = CollisionComponent::new();
  let mut gravity_component = GravityComponent::new(0.01);
  let mut event_component = EventComponent::new();

  let mut floor = Floor::new();

  let mut playable_character = PlayableCharacter::new(Sprite::new(
    Rectangle::new(0.0, 0.5, 0.2, 0.3, COLORS::White.into()),
    Texture::new("witch", TextureOptions::default()),
  ));

  let mut random_entities = Vec::<PlayableCharacter>::new();

  playable_character.load_components(&mut collision_component);
  playable_character.load_components(&mut gravity_component);
  playable_character.load_components(&mut event_component);
  floor.load_components(&mut collision_component);
  bowtie.load_entity(&mut floor);
  bowtie.load_entity(&mut playable_character);

  bowtie.prep_for_render();

  while !window.should_close() {
    window.swap_buffers();
    glfw_instance.poll_events();

    let collision_direction = collision_component
      .get_entity_collision_direction(&mut playable_character);
    bowtie.update_entities();
    let is_collided =
      collision_component.get_is_collided(&mut playable_character);
    if is_collided {
      playable_character.set_collision_direction(collision_direction);
    } else {
      playable_character.set_collision_direction(collision_direction);
    }
    bowtie.draw_entities();

    for (_, event) in glfw::flush_messages(&events) {
      futures::executor::block_on(handle_player_events(
        event.to_owned(),
        &mut playable_character,
      ));
      match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, _, _) => {
          window.set_should_close(true);
        }
        glfw::WindowEvent::Key(glfw::Key::O, _, _, _) => {
          random_entities.push(PlayableCharacter::new(Sprite::new(
            Rectangle::new(0.0, 0.5, 0.2, 0.3, COLORS::White.into()),
            Texture::new("character", TextureOptions::default()),
          )));
          let id = random_entities.len() - 1;
          bowtie.load_entity(&mut random_entities[id]);
        }
        _ => {}
      }
    }
  }

  window.close();
}
