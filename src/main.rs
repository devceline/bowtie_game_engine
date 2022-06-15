extern crate gl;
extern crate glfw;

mod gl_utils;

use gl_utils::shader_creator::{DataType, Shader, ShaderProgram, Uniform, VertexShaderAttribute};

use gl_utils::vertex_array_object_handler::VertexArrayObject;

use gl_utils::gl_error_reader;

use std::{mem::size_of_val, os::raw::c_void};

use glfw::Context;

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
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

  let (mut window, events) = glfw
    .create_window(300, 300, "rust game engine", glfw::WindowMode::Windowed)
    .expect("Failed to create glfw window");

  window_setup(&mut glfw, &mut window);
  gl_error_reader::init_debug_callback();

  let elements: [i32; 3] = [0, 1, 2];
  let vertices: [f32; 15] = [
    0.0, 0.5, 1.0, 0.8, 0.3, 0.5, -0.5, 0.5, 0.2, 1.0, -0.5, -0.5, 0.0, 1.0, 0.8,
  ];

  // Initialize a vao to handle gl data
  VertexArrayObject::new();

  // Initialize a program and load a vertex and fragment shader
  let mut program = ShaderProgram::new();
  program.load_shaders(vec![
    Shader::VertexShader(
      String::from("main"),
      vec![
        VertexShaderAttribute::new(String::from("position"), DataType::Float32, 2, 5, true, 0),
        VertexShaderAttribute::new(
          String::from("targetColor"),
          DataType::Float32,
          3,
          5,
          true,
          2,
        ),
      ],
    ),
    Shader::FragmentShader(String::from("main")),
  ]);

  unsafe {
    // Buffer data
    let mut vba: u32 = 0;
    let mut ebo: u32 = 0;

    gl::GenBuffers(1, &mut vba);
    gl::BindBuffer(gl::ARRAY_BUFFER, vba);
    gl::BufferData(
      gl::ARRAY_BUFFER,
      size_of_val(&vertices) as isize,
      vertices.as_ptr() as *const c_void,
      gl::STATIC_DRAW,
    );

    gl::GenBuffers(1, &mut ebo);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(
      gl::ELEMENT_ARRAY_BUFFER,
      size_of_val(&elements) as isize,
      elements.as_ptr() as *const c_void,
      gl::STATIC_DRAW,
    );
  }

  program.use_program();

  program.set_uniform(Uniform {
    name: String::from("triangleColor"),
    data_type: DataType::Float32,
    count: 3,
    values: vec![0.8, 0.2, 0.5],
  });

  while !window.should_close() {
    window.swap_buffers();
    glfw.poll_events();

    unsafe {
      gl::DrawElements(
        gl::TRIANGLES,
        3,
        gl::UNSIGNED_INT,
        0 as *const gl::types::GLvoid,
      );
    }

    for (_, event) in glfw::flush_messages(&events) {
      match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
          window.set_should_close(true)
        }
        _ => {}
      }
    }
  }
}
