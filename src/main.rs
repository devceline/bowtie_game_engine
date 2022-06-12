extern crate gl;
extern crate glfw;

mod gl_utils;

use gl_utils::shader_creator::{AttribDataType, Shader, ShaderProgram, VertexShaderAttribute};

use gl_utils::vertex_array_object_handler::VertexArrayObject;

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
        .create_window(300, 300, "hi twitter", glfw::WindowMode::Windowed)
        .expect("Failed to create glfw window");

    window_setup(&mut glfw, &mut window);

    let mut buf1: u32 = 0;
    let vertices: [f32; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

    // Initialize a vao to handle gl data
    VertexArrayObject::new();

    // Initialize a program and load a vertex and fragment shader
    let mut program = ShaderProgram::new();
    program.load_shaders(vec![
        Shader::VertexShader(
            String::from("main"),
            vec![VertexShaderAttribute {
                name: String::from("position"),
                normalized: true,
                stride: 0,
                size: 2,
                attrib_data_type: AttribDataType::Float,
                pointer: ptr::null(),
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
