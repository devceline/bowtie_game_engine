extern crate bowtie;
extern crate futures;
extern crate gl;
extern crate glfw;
extern crate rand;

use bowtie::{
  BowTie, Entity, StandardEntity, LoadableTexture, Rectangle, Sprite, Texture, TextureOptions,
  COLORS, Message, init_debug_callback
};

// use game_objects::{floor::Floor, playable_character::{PlayableCharacter, MessageReciever}};
use glfw::Context;

fn message_reciever<'s>(entity: &mut dyn Entity<'s>, message: Message) {}

// async fn handle_player_events<'a>(
//   event: glfw::WindowEvent,
//   character: &mut PlayableCharacter<'a>,
// ) {
//   // println!("x: {}, y: {}, y+height: {}", character.get_x(), character.get_y(), character.get_y() + character.get_height());
//   futures::join!(character.respond_to_event(&event));
// }

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

  // init_debug_callback();

  // let mut collision_component = CollisionComponent::new();
  // let mut gravity_component = GravityComponent::new(0.005);
  // let mut event_component = EventComponent::new();
  let mut random_entities = Vec::<StandardEntity>::new();
  random_entities.reserve(2000);

   let en_texture = Texture::new("character", TextureOptions::default());

   bowtie.load_entity(StandardEntity::new(Sprite::new(
     Rectangle::new(0.0, -0.5, 0.2, 0.3, COLORS::Red.into()),
       Texture::from(&en_texture)), 2.0));

  bowtie.load_entity(StandardEntity::new(Sprite::new(
    Rectangle::new(0.0, 0.0, 0.2, 0.3, COLORS::Red.into()),
      Texture::new("floor", TextureOptions::default())), 2.0));
  
  bowtie.load_entity(StandardEntity::new(Sprite::new(
    Rectangle::new(0.0, 0.5, 0.2, 0.3, COLORS::Red.into()),
      Texture::new("witch", TextureOptions::default())), 2.0));


  bowtie.prep_for_render();

  while !window.should_close() {
    window.swap_buffers();
    glfw_instance.poll_events();
    bowtie.draw_entities();

    for (_, event) in glfw::flush_messages(&events) {
      // futures::executor::block_on(handle_player_events(
      //   event.to_owned(),
      //   &mut playable_character,
      // ));
      match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, _, _) => {
          window.set_should_close(true);
        }
        glfw::WindowEvent::Key(glfw::Key::O, _, glfw::Action::Press, _) => {
          for _ in 0..20 {
            // random_entities.push();
            // let id = random_entities.len() - 1;
            // random_entities[id].load_components(&mut gravity_component);
            // bowtie.load_entity(random_entities[id]);
            bowtie.load_entity(
              StandardEntity::new(Sprite::new(
              Rectangle::new(
                (rand::random::<f32>() % 1.0) - 0.5,
                (rand::random::<f32>() % 1.0) - 0.5,
                0.2,
                0.3,
                COLORS::White.into(),
              ),
              Texture::none(),
            ), 2.0)
            )
          }
        }
        _ => {}
      }
    }
  }

  window.close();
}
