extern crate glfw;
extern crate gl;

mod gl_utils;

use gl_utils::shader_creator::{create_shader_program, Shader, VertexShaderAttribute, AttribDataType, use_gl_program};

use std::{mem::size_of_val, os::raw::c_void, ptr};

use glfw::Context;

fn main() {
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();



  let (mut window, events) = 
    glfw.create_window(300, 300, "hi twitter", glfw::WindowMode::Windowed)
    .expect("Failed to create glfw window");

  window.make_current();

  gl::load_with(|s| glfw.get_proc_address_raw(s));

  glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
  glfw.window_hint(glfw::WindowHint::ContextVersionMinor(2));
  glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
  glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

  window.make_current();
  window.set_key_polling(true);

  let mut buf1: u32 = 0;
  let vertices: [f32; 6] = [
     0.0,  0.5, 
     0.5, -0.5, 
    -0.5, -0.5  
  ];


  unsafe {
    // Vertex Array Object
    let mut vao: u32 = 0;
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);

    // Buffer data
    gl::GenBuffers(1, &mut buf1);
    gl::BindBuffer(gl::ARRAY_BUFFER, buf1);
    gl::BufferData(gl::ARRAY_BUFFER, size_of_val(&vertices) as isize, vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);

    let program_id = create_shader_program(vec![Shader::VertexShader(String::from("main"), vec![VertexShaderAttribute {
      name: String::from("position"),
      normalized: true,
      stride: 0,
      size: 2,
      attrib_data_type: AttribDataType::Float,
      pointer: ptr::null()
    }]), Shader::FragmentShader(String::from("main"))]);

    use_gl_program(program_id);

    let err = gl::GetError();
    println!("{}", err);
  }


  while !window.should_close() {
    window.swap_buffers();
    glfw.poll_events();

    unsafe {
      gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }

    for(_, event) in glfw::flush_messages(&events) {
      println!("{:?}", event);
      match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
          window.set_should_close(true)
        },
        _ => {},
      }
    }
  }
}
