extern crate futures;
extern crate gl;
extern crate glfw;
extern crate png;

mod game_objects;
mod general;
mod gl_utils;
mod god_object;
mod math;
mod rendering;
mod shapes;
mod sprites;
mod components;

use components::collide::CollisionComponent;
use glfw::Context;

use general::color::COLORS;
use gl_utils::gl_error_reader;
use gl_utils::gl_texture::{Texture, TextureOptions};
use god_object::entity::Entity;
use game_objects::playable_character::PlayableCharacter;
use god_object::god_object::BowTie;
use shapes::rectangle::Rectangle;
use sprites::sprite::Sprite;


async fn handle_events<'a>(
  event: glfw::WindowEvent,
  character: &mut Sprite<'a, Rectangle>,
) {
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
    .create_window(1031, 540, "rust game engine", glfw::WindowMode::Windowed)
    .expect("Failed to create window");
  window_setup(&mut glfw_instance, &mut window);

  gl_error_reader::init_debug_callback();
  let mut god_object = BowTie::new();


  // let sky = Sprite::new(
  //   Rectangle::new(-1.0, 1.0, 2.0, 2.0, COLORS::White.into()),
  //   Texture::new("sky", TextureOptions::default()),
  // );
  // let floor = Sprite::new(
  //   Rectangle::new(-1.0, -0.5, 2.0, 0.5, COLORS::White.into()),
  //   Texture::new("floor", TextureOptions::default()),
  // );
  // let mut game_world = GameWorld::new(floor, sky);


  let mut playable_character = PlayableCharacter::new(Sprite::new(Rectangle::new(
      -0.5, -0.5, 0.3, 0.2, COLORS::White.into()
      ), Texture::new("character", TextureOptions::default())));
  let mut collision_component = CollisionComponent::new();

  let mut playable_character2 = PlayableCharacter::new(Sprite::new(Rectangle::new(
      0.5, -0.5, 0.3, 0.2, COLORS::White.into()
      ), Texture::new("character", TextureOptions::default())));

  playable_character.load_components(&mut collision_component);
  // playable_character2.load_components(&mut collision_component);

  god_object.load_entity(&mut playable_character);
  god_object.load_entity(&mut playable_character2);

  god_object.prep_for_render();

  while !window.should_close() {
    window.swap_buffers();
    glfw_instance.poll_events();

    god_object.update_entities();
    let is_collided = collision_component.get_is_collided(&mut playable_character);
    if is_collided {
      playable_character.set_color_overlay(COLORS::Red.into());
    }
    else {
      playable_character.set_color_overlay(COLORS::White.into());
    }
    god_object.draw_entities();

    for (_, event) in glfw::flush_messages(&events) {
      // futures::executor::block_on(handle_events(
      //   event.to_owned(),
      //   &mut character,
      // ));
      match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, _, _) => {
          window.set_should_close(true);
        }
        glfw::WindowEvent::Key(glfw::Key::Left, _, _, _) => {
          playable_character.move_left();
        }
        glfw::WindowEvent::Key(glfw::Key::Right, _, _, _) => {
          playable_character.move_right();
        }
        _ => {}
      }
    }
  }

  window.close();
}
