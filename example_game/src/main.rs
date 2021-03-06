extern crate bowtie;
extern crate futures;
extern crate gl;
extern crate glfw;
extern crate rand;

mod game_objects;

use bowtie::{
  premade_components::{CollisionComponent, GravityComponent},
  BowTie, Entity, LoadableTexture, Rectangle, Sprite, Texture, TextureOptions,
  COLORS,
};

use game_objects::{floor::Floor, playable_character::PlayableCharacter};
use glfw::Context;

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

  let mut bowtie = BowTie::new();

  let mut collision_component = CollisionComponent::new();
  let mut gravity_component = GravityComponent::new(0.005);
  let en_texture = Texture::new("character", TextureOptions::default());
  en_texture.load_texture();
  let mut floor = Floor::new();

  let mut playable_character = PlayableCharacter::new(Sprite::new(
    Rectangle::new(0.0, 0.5, 0.2, 0.3, COLORS::White.into()),
    Texture::new("witch", TextureOptions::default()),
  ));

  let mut random_entities = Vec::<PlayableCharacter>::new();
  random_entities.reserve(2000);

  let mut obstacle = PlayableCharacter::new(Sprite::new(
    Rectangle::new(0.0, -0.4, 0.1, 0.1, COLORS::White.into()),
    Texture::new("dirt", TextureOptions::default()),
  ));

  playable_character.load_components(&mut collision_component);
  obstacle.load_components(&mut collision_component);
  playable_character.load_components(&mut gravity_component);
  floor.load_components(&mut collision_component);
  bowtie.load_entity(&mut obstacle);
  bowtie.load_entity(&mut floor);
  bowtie.load_entity(&mut playable_character);

  random_entities.push(PlayableCharacter::new(Sprite::new(
    Rectangle::new(0.0, 0.5, 0.2, 0.3, COLORS::White.into()),
    Texture::from(&en_texture),
  )));
  let id = random_entities.len() - 1;
  bowtie.load_entity(&mut random_entities[id]);

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
        glfw::WindowEvent::Key(glfw::Key::O, _, glfw::Action::Press, _) => {
          println!("Handling {} enemies", random_entities.len());
          for _ in 0..20 {
            random_entities.push(PlayableCharacter::new(Sprite::new(
              Rectangle::new(
                (rand::random::<f32>() % 2.0) - 1.0,
                (rand::random::<f32>() % 2.0) - 1.0,
                0.2,
                0.3,
                COLORS::White.into(),
              ),
              Texture::from(&en_texture),
            )));
            let id = random_entities.len() - 1;
            bowtie.load_entity(&mut random_entities[id]);
          }
        }
        _ => {}
      }
    }
  }

  window.close();
}
