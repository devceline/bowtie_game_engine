extern crate gl;

use std::{fs, collections::HashMap, ffi::CString};

enum AttribDataType {
  Float,
  Int
}

struct VertexShaderAttribute {
  name: String,
  attrib_data_type: AttribDataType,
  size: i32,
  stride: i32,
  normalized: bool
  pointer: *const std::os::raw::c_void
}

enum Shader {
  VertexShader(String, Vec<VertexShaderAttribute>),
  FragmentShader(String)
}

fn get_c_string(original_string: String) -> CString {
  return CString::new(original_string).expect("Could not convert to c string");
}

fn get_shader_location(shader: &Shader) -> String {
  let mut base_url = String::from("./shaders/");
  match shader {
    Shader::VertexShader(name, _attributes) => {
      base_url.push_str("vertex/");
      base_url.push_str(&name);
    },
    Shader::FragmentShader(name) => {
      base_url.push_str("fragment/");
      base_url.push_str(&name);
    }
  }

  return base_url;
}

fn load_shader_src(shader: &Shader, id: u32) {
  let source_code = fs::read_to_string(get_shader_location(shader))
    .expect("Could not locate shader");

  let source_code_ptr: *const *const i8 = &(source_code.as_ptr() as *const i8);

  unsafe { gl::ShaderSource(id, 1, source_code_ptr, std::ptr::null()) }
}

/**
 * Takes in a shader, loads its source code and 
 * returns its id after compilation
 */
fn init_shader(shader: &Shader) -> u32 {

  let shader_id = match shader {
    Shader::VertexShader(ref _name, ref _attributes) => 
      unsafe { gl::CreateShader(gl::VERTEX_SHADER) },
    Shader::FragmentShader(ref _name) => 
      unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) }
  };

  load_shader_src(shader, shader_id);

  unsafe { gl::CompileShader(shader_id) }

  return shader_id;
}

fn create_shader_program(shaders: Vec<Shader> ) {
  let mut shader_map: HashMap<u32, Shader> = HashMap::new();

  for shader in shaders {
    match shader {
      Shader::VertexShader(ref _name, ref _attributes) => {
        shader_map.insert(init_shader(&shader), shader);
      },
      Shader::FragmentShader(ref _name) => {
        shader_map.insert(init_shader(&shader), shader);
      }
    }
  }

  unsafe {
    let program = gl::CreateProgram();
    for (id, _shader) in &shader_map {
      gl::AttachShader(program, id.to_owned())
    }

    for (id, shader) in shader_map {
      match shader {
        Shader::VertexShader(name, attributes) => {
          for attribute in attributes {
              let attrib_name = 
                get_c_string(attribute.name);
              let attrib_location = 
                gl::GetAttribLocation(program, attrib_name.as_ptr()) as u32;
              let gl_data_type = match attribute.attrib_data_type {
                AttribDataType::Float => gl::FLOAT,
                AttribDataType::Int => gl::UNSIGNED_INT,
              };
              let gl_normalized = 
                if attribute.normalized { gl::TRUE } else { gl::FALSE };

              gl::VertexAttribPointer(
                attrib_location, 
                attribute.size, 
                gl_data_type,
                gl_normalized,
                attribute.stride,
                attribute.pointer
              );

              gl::EnableVertexAttribArray(attrib_location);
          }
        },
        Shader::FragmentShader(_name) => {},
      }
    }

    gl::LinkProgram(program);

  }
}

fn use_program(program_id: u32) {
  unsafe {
    gl::UseProgram(program_id);
  }
}







