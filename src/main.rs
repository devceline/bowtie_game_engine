extern crate gl;
extern crate glfw;

mod gl_utils;

use gl_utils::shader_creator::{GlDataType, Shader, ShaderProgram, Uniform, VertexShaderAttribute};

use gl_utils::vertex_array_object_handler::VertexArrayObject;

use std::mem::size_of;
use std::{mem::size_of_val, os::raw::c_void, ptr};

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

    let mut buf1: u32 = 0;

    let vertices: [f32; 15] = [
      0.0, 0.5, 1.0, 0.8, 0.3,
      0.5, -0.5, 0.5, 0.2, 1.0,
      -0.5, -0.5, 0.0, 1.0, 0.8
    ];

    // Initialize a vao to handle gl data
    VertexArrayObject::new();

    // Initialize a program and load a vertex and fragment shader
    let mut program = ShaderProgram::new();
    let mut tmp = 2*size_of::<f32>();
    program.load_shaders(vec![
        Shader::VertexShader(
            String::from("main"),
            vec![VertexShaderAttribute {
                name: String::from("position"),
                data_type: GlDataType::Float,
                size: 2,
                normalized: true,
                stride: 5*size_of::<f32>() as i32,
                pointer: ptr::null(),
            }, VertexShaderAttribute {
              name: String::from("color"),
              size:3,
              data_type: GlDataType::Float,
              stride: 5*size_of::<f32>() as i32,
              normalized: true,
              pointer: (&mut tmp) as *mut _ as *mut c_void
            }],
        ),
        Shader::FragmentShader(String::from("main")),
    ]);

    unsafe {
        // Buffer data
        gl::GenBuffers(1, &mut buf1);
        gl::BindBuffer(gl::ARRAY_BUFFER, buf1);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&vertices) as isize,
            vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        let err = gl::GetError();
        println!("{}", err);
    }

    program.use_program();
    program.set_uniform(Uniform {
        name: String::from("triangleColor"),
        data_type: GlDataType::Float,
        count: 3,
        values: vec![0.8, 0.2, 0.5],
    });

    while !window.should_close() {
        window.swap_buffers();
        glfw.poll_events();

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}
