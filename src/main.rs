extern crate gl;
extern crate glfw;
extern crate png;

use glfw::Context;

mod gl_utils;

use gl_utils::element_array_buffer::ElementArrayBuffer;
use gl_utils::gl_error_reader;
use gl_utils::gl_translation::{DataType, DrawingMode, UsageMode};
use gl_utils::shader_creator::{Shader, ShaderProgram, Uniform, VertexShaderAttribute};
use gl_utils::vertex_array_buffer::VertexArrayBuffer;
use gl_utils::vertex_array_object_handler::VertexArrayObject;

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

  // Initialize a vao to handle gl data
  VertexArrayObject::new();

  // Initialize a program and load a vertex and fragment shader
  let mut program = ShaderProgram::new();
  program.load_shaders(vec![
    Shader::VertexShader(
      String::from("main"),
      vec![
        VertexShaderAttribute::new(String::from("position"), DataType::Float32, 2, 7, true, 0),
        VertexShaderAttribute::new(String::from("targetColor"),DataType::Float32, 3, 7, true, 2),
        VertexShaderAttribute::new(String::from("tex_cords_in"), DataType::Float32, 2, 7, true, 5),
      ],
    ),
    Shader::FragmentShader(String::from("main")),
  ]);

  // Keeping a variable regardless of use to prevent drop() being called.
  let _vertex_array_buffer = VertexArrayBuffer::<f32>::new(
    vec![
      // X   Y    R    G    B   T_X  T_Y
      -1.0, 0.5, 1.0, 0.0, 1.0, 0.0, 0.0, // vertex 1
      1.0, 0.5, 1.0, 0.0, 1.0, 1.0, 0.0, // vertex 2
      1.0, -0.5, 1.0, 0.0, 1.0, 1.0, 1.0, // vertex 3
      -1.0, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0 // vertex 3
    ],
    DataType::Float32,
    UsageMode::StaticDraw,
  );

  let element_buffer = ElementArrayBuffer::new(
    vec![0, 1, 2, 2, 3, 0],
    DataType::UnsignedInt,
    UsageMode::StaticDraw,
  );

  program.use_program();

  unsafe {
    let mut tex = 0;

    let decoder = png::Decoder::new(std::fs::File::open("images/pride_flag.png").unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    let (color, _) = reader.output_color_type();
    println!("Color: {:?}", color);
    reader.next_frame(&mut buf).unwrap();

    gl::GenTextures(1, &mut tex);
    gl::ActiveTexture(gl::TEXTURE0);
    gl::BindTexture(gl::TEXTURE_2D, tex);


    gl::TexImage2D(
      gl::TEXTURE_2D,
      0, gl::RGBA as i32,
      info.width as i32,
      info.height as i32,
      0, gl::RGBA,
      gl::UNSIGNED_BYTE,
      buf.as_ptr() as *const gl::types::GLvoid
    );


    program.set_uniform(Uniform {
      name: String::from("tex"),
      data_type: DataType::Int,
      count: 1,
      values: vec![0]
    });

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

  }

  while !window.should_close() {
    window.swap_buffers();
    glfw.poll_events();

    element_buffer.draw(DrawingMode::Triangles);

    for (_, event) in glfw::flush_messages(&events) {
      match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
          window.set_should_close(true)
        }
        _ => {}
      }
    }
  }

  window.close();
}
