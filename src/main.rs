extern crate gl;
extern crate glfw;
extern crate png;

mod game_objects;
mod general;
mod gl_utils;
mod math;
mod rendering;
mod shapes;
mod sprites;

use glfw::Context;

use general::color::COLORS;

use gl_utils::gl_error_reader;
use gl_utils::gl_texture::{LoadableTexture, Texture, TextureOptions};
use gl_utils::gl_translation::{DataType, DrawingMode, UsageMode};
use gl_utils::shader_creator::{ Shader, ShaderProgram, VertexShaderAttribute};
use gl_utils::vertex_array_object_handler::VertexArrayObject;
use gl_utils::uniform::UniformMatrixFloat;

use rendering::drawer::Drawer;
use shapes::rectangle::Rectangle;
use sprites::sprite::Sprite;

use game_objects::game_world::GameWorld;

use crate::math::matrix::Matrix;

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
}

fn get_program() -> ShaderProgram {
  let mut program = ShaderProgram::new();
  program.load_shaders(vec![
    Shader::VertexShader(
      String::from("main"),
      vec![
        VertexShaderAttribute::new(
          String::from("position"),
          DataType::Float32,
          2,
          9,
          true,
          0,
        ),
        VertexShaderAttribute::new(
          String::from("targetColor"),
          DataType::Float32,
          4,
          9,
          true,
          2,
        ),
        VertexShaderAttribute::new(
          String::from("tex_cords_in"),
          DataType::Float32,
          2,
          9,
          true,
          6,
        ),
        VertexShaderAttribute::new(
          String::from("tex_id"),
          DataType::Float32,
          1,
          9,
          true,
          8,
        ),
      ],
    ),
    Shader::FragmentShader(String::from("main")),
  ]);
  program
}

fn main() {
  let mut glfw_instance = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  let (mut window, events) = glfw_instance
    .create_window(700, 700, "rust game engine", glfw::WindowMode::Windowed)
    .expect("Failed to create window");
  window_setup(&mut glfw_instance, &mut window);

  gl_error_reader::init_debug_callback();

  // Initialize a vao to handle gl data
  let _vao = VertexArrayObject::new();

  // Initialize a program and load a vertex and fragment shader
  let program = get_program();

  // Transformation test
  let mut drawer = Drawer::new(UsageMode::StaticDraw, &program);

  let enemy_texture = Texture::new("enemy", TextureOptions::default());
  enemy_texture.load_texture();

  let sky = Sprite::new(
    Rectangle::new(-1.0, 1.0, 2.0, 2.0, COLORS::White.into()),
    Texture::new("sky", TextureOptions::default()),
  );
  let floor = Sprite::new(
    Rectangle::new(-1.0, -0.5, 2.0, 0.5, COLORS::White.into()),
    Texture::new("floor", TextureOptions::default()),
  );

  let game_world = GameWorld::new(floor, sky);

  let mut character = Sprite::new(
    Rectangle::new(-0.7, -0.6, 0.3, 0.4, COLORS::White.into()),
    Texture::new("character", TextureOptions::default()),
  );

  let mut enemies = Vec::<Sprite<Rectangle>>::new();
  // for i in 0..100 {
  //   enemies.push(Sprite::new(
  //     Rectangle::new(
  //       (i as f32 / 100.0) - 0.5,
  //       (i as f32 / 100.0) - 0.5,
  //       0.2,
  //       0.3,
  //       COLORS::White.into(),
  //     ),
  //     Texture::from(&enemy_texture),
  //   ));
  // }
  // for i in 0..100 {
  //   enemies.push(Sprite::new(
  //     Rectangle::new(
  //       (i as f32 / 100.0) - 0.3,
  //       (i as f32 / 100.0) - 0.5,
  //       0.2,
  //       0.3,
  //       COLORS::White.into(),
  //     ),
  //     Texture::from(&enemy_texture),
  //   ));
  // }
  // for i in 0..100 {
  //   enemies.push(Sprite::new(
  //     Rectangle::new(
  //       (i as f32 / 100.0) - 0.1,
  //       (i as f32 / 100.0) - 0.5,
  //       0.2,
  //       0.3,
  //       COLORS::White.into(),
  //     ),
  //     Texture::from(&enemy_texture),
  //   ));
  // }

  let mut fireball = Sprite::new(
    Rectangle::new(-0.7, -0.6, 0.15, 0.1, COLORS::Red.into()),
    Texture::new("fireball", TextureOptions::default()),
  );

  let mut fireball_moving = false;

  drawer.load_sprite_dynamic(&game_world);
  drawer.load_sprite_dynamic(&character);
  drawer.load_sprite_dynamic(&fireball);

  for enemy in &enemies {
    drawer.load_sprite_dynamic(enemy);
  }

  program.use_program();

  let trans = Matrix::new(vec![
    vec![0.40808206181, -0.91294525073, 0.0, 0.0],
    vec![0.91294525073, 0.40808206181, 0.0, 0.0],
    vec![0.0, 0.0, 1.0, 0.0],
    vec![0.0, 0.0, 0.0, 1.0],
  ]);

  let trans_uniform = UniformMatrixFloat::new("trans", trans);
  program.set_uniform(&trans_uniform);


  drawer.prep_textures();

  let mut frames = 0;
  let mut now = std::time::Instant::now();

  while !window.should_close() {
    window.swap_buffers();
    glfw_instance.poll_events();

    drawer.clear_screen(COLORS::Black.into());
    drawer.draw(DrawingMode::Triangles);

    if now.elapsed().as_secs() < 1 {
      frames += 1;
    } else {
      println!("FPS: {frames}");
      frames = 0;
      now = std::time::Instant::now();
    }

    if fireball_moving {
      if !fireball.move_right(0.1) {
        fireball_moving = false;
        fireball.set_x(character.get_x());
        fireball.set_y(character.get_y() - 0.2);
        drawer.unload_sprite_dynamic(&fireball);
      }
    } else {
      fireball.set_x(character.get_x());
      fireball.set_y(character.get_y() - 0.2);
      character.set_color_overlay(COLORS::White.into());
      drawer.unload_sprite_dynamic(&fireball);
    }

    for (_, event) in glfw::flush_messages(&events) {
      match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
          window.set_should_close(true);
        }
        glfw::WindowEvent::Key(glfw::Key::Right, _, glfw::Action::Repeat, _) => {
          character.move_right(0.02);
        }
        glfw::WindowEvent::Key(glfw::Key::Right, _, glfw::Action::Press, _) => {
          character.move_right(0.02);
        }
        glfw::WindowEvent::Key(glfw::Key::Left, _, glfw::Action::Repeat, _) => {
          character.move_left(0.02);
        }
        glfw::WindowEvent::Key(glfw::Key::Left, _, glfw::Action::Press, _) => {
          character.move_left(0.02);
        }
        glfw::WindowEvent::Key(glfw::Key::Up, _, glfw::Action::Repeat, _) => {
          character.move_up(0.02);
        }
        glfw::WindowEvent::Key(glfw::Key::Down, _, glfw::Action::Repeat, _) => {
          character.move_down(0.02);
        }
        glfw::WindowEvent::Key(glfw::Key::Space, _, glfw::Action::Press, _) => {
          fireball_moving = true;
          character.set_color_overlay(COLORS::Red.into());
          drawer.load_sprite_dynamic(&fireball);
        }
        glfw::WindowEvent::Key(glfw::Key::K, _, glfw::Action::Press, _) => {
          drawer.unload_sprite_dynamic(&character);
        }
        _ => {}
      }
    }
  }

  window.close();
}
