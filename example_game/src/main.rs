extern crate bowtie;
extern crate futures;
extern crate gl;
extern crate glfw;
extern crate rand;

mod components;

use std::{collections::HashMap, sync::Arc};

use components::rand_move::RandMove;
use rand::Rng;

use bowtie::{
  init_debug_callback, math,
  premade_components::{CollisionComponent, GravityComponent, KeyboardMoveComponent},
  BowTie, Direction, Entity, Message, Rectangle, Sprite, StandardComponent,
  StandardEntity, Texture, TextureOptions, COLORS,
};

use glfw::Context;

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

  glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

  init_debug_callback();

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

  let mut collision = CollisionComponent::new();
  let rand_move1 = RandMove::new();
  let mut gravity = GravityComponent::new(0.002);
  let keyboard_move = KeyboardMoveComponent::new(0.02, 0.0, 0.4);

  let collision_comp = collision.component();
  let rand_comp = rand_move1.component();
  let gravity_comp = gravity.component();
  let keyboard_move_comp = keyboard_move.component();

  let mut bowtie = BowTie::new();

  let en_texture = Texture::new("witch", TextureOptions::default());

  let playable_character = bowtie.load_entity(StandardEntity::new(Sprite::new(Rectangle::new(0.0, 0.0, 0.2, 0.3, COLORS::White.into()), Texture::from(&en_texture)), 0.0));
  playable_character.load_components(collision_comp.to_owned());
  playable_character.load_components(keyboard_move_comp.to_owned());

  bowtie.prep_for_render();

  //TODO: Make hollow rectangle
  let line_thickness = 0.01;
  let line_vectors = [
    // (-1.0, 1.0, 2.0, line_thickness),
    // (-1.0, 1.0, line_thickness, 2.0),
    (0.7 - line_thickness, 1.0, line_thickness * 10.0, 2.0),
    // (-1.0, -1.0 + line_thickness, 2.0, line_thickness)
  ];
  for line in line_vectors {
    let (x, y, w, h) = line;
    let created_line = bowtie.load_entity(StandardEntity::new(
      Sprite::new(
        Rectangle::new(x, y, w, h, COLORS::Red.into()),
        Texture::none(),
      ),
      0.0,
    ));
    created_line.load_components(collision_comp.to_owned());
  }

  while !window.should_close() {
    window.swap_buffers();
    glfw_instance.poll_events();
    bowtie.update_entities();
    bowtie.draw_entities();

    for (_, event) in glfw::flush_messages(&events) {
      keyboard_move.listen_for_event(&event);
      match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, _, _) => {
          window.set_should_close(true);
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
            2.0,
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
              2.0,
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

  window.close();
}
