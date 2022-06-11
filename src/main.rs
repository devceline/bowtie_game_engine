extern crate glfw;
extern crate gl;

mod gl_utils;

macro_rules! c_str {
    ($s:expr) => { {
        concat!($s, "\0").as_ptr() as *const i8
    } }
}

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

  let vert_src = std::fs::read_to_string("./shaders/vertex.glsl")
    .expect("Could not load vertex shader");
  let frag_src = std::fs::read_to_string("./shaders/fragment.glsl")
    .expect("Could not load vertex shader") ;


  let vert_src_ptr: *const *const i8 = &(vert_src.as_ptr() as *const i8);
  let frag_src_ptr: *const *const i8 = &(frag_src.as_ptr() as *const i8);

  unsafe {
    // Vertex Array Object
    let mut vao: u32 = 0;
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);

    // Buffer data
    gl::GenBuffers(1, &mut buf1);
    gl::BindBuffer(gl::ARRAY_BUFFER, buf1);
    gl::BufferData(gl::ARRAY_BUFFER, size_of_val(&vertices) as isize, vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);

    // Making the shading program
    let vert_shader = gl::CreateShader(gl::VERTEX_SHADER);
    gl::ShaderSource(vert_shader, 1, vert_src_ptr , ptr::null());
    gl::CompileShader(vert_shader);

    let frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    gl::ShaderSource(frag_shader, 1, frag_src_ptr , ptr::null());
    gl::CompileShader(frag_shader);

    let gl_shader_program = gl::CreateProgram();
    gl::AttachShader(gl_shader_program, vert_shader);
    gl::AttachShader(gl_shader_program, frag_shader);

    gl::LinkProgram(gl_shader_program);
    gl::UseProgram(gl_shader_program);


    let pos_attrib = gl::GetAttribLocation(gl_shader_program, c_str!("position") ) as u32;
    gl::VertexAttribPointer(pos_attrib, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());

    gl::EnableVertexAttribArray(pos_attrib);

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
