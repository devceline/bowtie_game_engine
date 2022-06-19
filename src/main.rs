extern crate gl;
extern crate glfw;
extern crate png;

mod general;
mod gl_utils;
mod math;
mod rendering;
mod shapes;
mod sprites;

use glfw::Context;

use general::color::COLORS;
use gl_utils::gl_error_reader;
use gl_utils::gl_texture::{Texture, TextureOptions};
use gl_utils::gl_translation::{DataType, DrawingMode, UsageMode};
use gl_utils::shader_creator::{Shader, ShaderProgram, VertexShaderAttribute};
use gl_utils::vertex_array_object_handler::VertexArrayObject;
use rendering::drawer::Drawer;
use shapes::rectangle::Rectangle;
use sprites::sprite::Sprite;

fn window_setup(glfw: &mut glfw::Glfw, window: &mut glfw::Window) {
  window.make_current();

  gl::load_with(|s| glfw.get_proc_address_raw(s));

  glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
  glfw.window_hint(glfw::WindowHint::ContextVersionMinor(2));
  glfw.window_hint(glfw::WindowHint::OpenGlProfile(
    glfw::OpenGlProfileHint::Core,
  ));
  glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

  window.make_current();
  window.set_key_polling(true);
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

  let mut drawer = Drawer::new(UsageMode::StaticDraw, &program);

  let sky = Sprite::new(Rectangle {
    x: -1.0,
    y: 1.0,
    width: 2.0,
    height: 2.0,
    color: COLORS::White.into()
  }, Texture::new("sky", TextureOptions::defaults(), &program));
  let floor = Sprite::new(Rectangle {
    x: -1.0,
    y: -0.5,
    width: 2.0,
    height: 0.5,
    color: COLORS::White.into(),
  }, Texture::new("floor", TextureOptions::defaults(), &program));
  let mut character = Sprite::new(Rectangle {
    x: -0.7,
    y: -0.6,
    width: 0.3,
    height: 0.4,
    color: COLORS::White.into(),
  }, Texture::new("character", TextureOptions::defaults(), &program));

  drawer.load_sprite_dynamic(&sky);
  drawer.load_sprite_dynamic(&floor);
  drawer.load_sprite_dynamic(&character);

  program.use_program();

  drawer.prep_textures();


  while !window.should_close() {
    window.swap_buffers();
    glfw_instance.poll_events();


    drawer.clear_screen(COLORS::Black.into());
    drawer.draw(DrawingMode::Triangles);


    for (_, event) in glfw::flush_messages(&events) {
      match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
          window.set_should_close(true);
        }
        glfw::WindowEvent::Key(glfw::Key::Right, _, glfw::Action::Repeat, _) => {
           character.move_right(0.02);
        }
        glfw::WindowEvent::Key(glfw::Key::Left, _, glfw::Action::Repeat, _) => {
          character.move_left(0.02);
        }
        glfw::WindowEvent::Key(glfw::Key::Up, _, glfw::Action::Repeat, _) => {
           character.move_up(0.02);
        }
        glfw::WindowEvent::Key(glfw::Key::Down, _, glfw::Action::Repeat, _) => {
          character.move_down(0.02);
        },
        glfw::WindowEvent::Key(glfw::Key::K, _, glfw::Action::Press, _) => {
          drawer.unload_sprite_dynamic(&character);
        }
        _ => {}
      }
    }
  }

  window.close();
}
