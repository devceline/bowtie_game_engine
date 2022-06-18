extern crate gl;
extern crate glfw;
extern crate png;

mod general;
mod gl_utils;
mod math;
mod rendering;
mod shapes;

use glfw::Context;

use general::color::COLORS;
use gl_utils::gl_texture::{Texture, TextureOptions};
use gl_utils::gl_translation::{DataType, DrawingMode, UsageMode};
use gl_utils::shader_creator::{Shader, ShaderProgram, VertexShaderAttribute};
use gl_utils::vertex_array_object_handler::VertexArrayObject;
use gl_utils::gl_error_reader;
use rendering::drawer::Drawer;
use shapes::{rectangle::Rectangle, shape::Shape};

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
      .create_window(300, 300, "rust game engine", glfw::WindowMode::Windowed)
      .expect("Failed to create window");
  window_setup(&mut glfw_instance, &mut window);

  gl_error_reader::init_debug_callback();


  // Initialize a vao to handle gl data
  VertexArrayObject::new();

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
          8,
          true,
          0,
        ),
        VertexShaderAttribute::new(
          String::from("targetColor"),
          DataType::Float32,
          4,
          8,
          true,
          2,
        ),
        VertexShaderAttribute::new(
          String::from("tex_cords_in"),
          DataType::Float32,
          2,
          8,
          true,
          6,
        ),
      ],
    ),
    Shader::FragmentShader(String::from("main")),
  ]);

  let rect = Rectangle {
    x: -0.4,
    y: 0.2,
    width: 1.1,
    height: 0.4,
    color: COLORS::Red.into(),
  };

  let mut drawer = Drawer::new(UsageMode::StaticDraw);
  drawer.load_shape(rect);

  program.use_program();

  let _texture1 =
    Texture::new("pride_flag", TextureOptions::defaults(), &program);

  let _texture2 = Texture::new("patrick", TextureOptions::defaults(), &program);

  while !window.should_close() {
    window.swap_buffers();
    glfw_instance.poll_events();

    drawer.draw(DrawingMode::Triangles);

    for (_, event) in glfw::flush_messages(&events) {
      match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
          window.set_should_close(true);
        }
        _ => {}
      }
    }

  }

  window.close();

}
