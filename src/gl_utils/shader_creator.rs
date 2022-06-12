extern crate gl;

use std::{fs, collections::HashMap, ffi::CString};

pub enum AttribDataType {
  Float,
  Int
}

pub struct VertexShaderAttribute {
  pub name: String,
  pub attrib_data_type: AttribDataType,
  pub size: i32,
  pub stride: i32,
  pub normalized: bool,
  pub pointer: *const std::os::raw::c_void
}

pub enum Shader {
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

  base_url.push_str(".glsl");

  return base_url;
}

fn load_shader_src(shader: &Shader, id: u32) {
  let location = get_shader_location(shader);
  let source_code = fs::read_to_string(location)
    .expect("Could not locate shader at location {}");

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

  get_shader_error(shader_id);


  return shader_id;
}

fn get_shader_error(shader_id: u32) {
  let mut status = -10;
  unsafe {
    gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut status);

    println!("Compilation status for shader {} is {}", shader_id, status);

    if (status as u8) != gl::TRUE {
      let mut buffer: Vec<i8> = Vec::new();
      let mut buffer_len = 0;
      gl::GetShaderInfoLog(shader_id, 512, &mut buffer_len, &mut buffer[0]);

      let log_string = 
        String::from_utf8(buffer.iter().map(|s| s.to_owned() as u8)
          .collect::<Vec<u8>>()).expect("Could not transform buffer to string");

      println!(
        "Shader with id {} could not compile because:\n {}", 
        shader_id,
        log_string
      );

    }
  }
}

pub fn create_shader_program(shaders: Vec<Shader>) -> u32  {
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
    let program_id = gl::CreateProgram();
    for (id, _shader) in &shader_map {
      gl::AttachShader(program_id, id.to_owned())
    }

    gl::LinkProgram(program_id);
    use_gl_program(program_id);

    for (_id, shader) in shader_map {
      match shader {
        Shader::VertexShader(_name, attributes) => {
          for attribute in attributes {
              let attrib_name = 
                get_c_string(attribute.name);
              let attrib_location = 
                gl::GetAttribLocation(program_id, attrib_name.as_ptr()) as u32;

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

    return program_id;

  }
}

pub fn use_gl_program(program_id: u32) {
  unsafe {
    gl::UseProgram(program_id);
  }
}

pub struct ShaderProgram {
}

impl ShaderProgram {
  pub fn new(shaders: Vec<Shader>) {
  }
    
}
