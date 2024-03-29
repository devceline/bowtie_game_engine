extern crate gl;

use std::{collections::HashMap, ffi::CString, fs};

use super::gl_error_reader::{GlError, GlErrorResult};
use super::gl_translation::{DataType, ToGl};
use super::uniform::SettableUniform;

#[derive(Clone)]
pub enum VertexShaderAttributeType {
  Vector,
  Matrix4,
}

#[derive(Clone)]
pub struct VertexShaderAttribute {
  pub name: String,
  pub data_type: DataType,
  pub size: i32,
  pub stride: i32,
  pub normalized: bool,
  pub offset: i32,
  pub original_offset: i32,
  pub attrib_type: VertexShaderAttributeType,
}

impl VertexShaderAttribute {
  pub fn new(
    name: String,
    data_type: DataType,
    size: i32,
    stride: i32,
    normalized: bool,
    offset: i32,
    attrib_type: VertexShaderAttributeType,
  ) -> VertexShaderAttribute {
    let attrib = VertexShaderAttribute {
      name,
      data_type,
      size,
      stride: ((data_type.get_size()) * stride),
      normalized,
      original_offset: offset,
      offset: ((data_type.get_size()) * offset),
      attrib_type,
    };

    return attrib;
  }
}

#[derive(Clone)]
pub enum Shader {
  VertexShader(String, Vec<VertexShaderAttribute>),
  FragmentShader(String),
}

impl Shader {
  fn get_name(&self) -> String {
    return match self {
      Shader::VertexShader(name, _attributes) => name.to_owned(),
      Shader::FragmentShader(name) => name.to_owned(),
    };
  }
}

fn get_c_string(original_string: String) -> CString {
  return CString::new(original_string).expect("Could not convert to c string");
}

#[derive(Clone)]
pub struct ShaderProgram {
  pub program_id: u32,
  shader_map: HashMap<u32, Shader>,
}

impl ShaderProgram {
  pub fn shell() -> ShaderProgram {
    ShaderProgram {
      program_id: 0,
      shader_map: HashMap::new(),
    }
  }
  pub fn new() -> ShaderProgram {
    unsafe {
      ShaderProgram {
        program_id: gl::CreateProgram(),
        shader_map: HashMap::new(),
      }
    }
  }

  pub fn set_uniform<T>(&self, uniform: &dyn SettableUniform<T>) {
    let uniform_name = get_c_string(uniform.get_name().to_owned());

    let uniform_location =
      unsafe { gl::GetUniformLocation(self.program_id, uniform_name.as_ptr()) };

    if uniform_location < 0 {
      panic!("Uniform {:?} was not found", uniform_name);
    }

    uniform.set_uniform(uniform_location);
  }

  fn get_shader_location(&self, shader: &Shader) -> String {
    let mut base_url = String::from(concat!(env!("OUT_DIR"), "/shaders/"));
    match shader {
      Shader::VertexShader(_name, _attributes) => {
        base_url.push_str("vertex/");
      }
      Shader::FragmentShader(_) => {
        base_url.push_str("fragment/");
      }
    }

    base_url.push_str(&shader.get_name());
    base_url.push_str(".glsl");

    return base_url;
  }

  fn load_shader_src(&self, shader: &Shader, id: u32) {
    let location = self.get_shader_location(shader);
    let source_code = fs::read_to_string(&location).expect(
      format!("Could not locate shader at location {}", &location).as_str(),
    );

    let source_code_ptr: *const *const i8 = &(source_code.as_ptr() as *const i8);

    unsafe { gl::ShaderSource(id, 1, source_code_ptr, std::ptr::null()) }
  }

  /**
   * Takes in a shader, loads its source code and
   * returns its id after compilation
   */
  fn init_shader(&self, shader: &Shader) -> u32 {
    let shader_id = match shader {
      Shader::VertexShader(ref _name, ref _attributes) => unsafe {
        gl::CreateShader(gl::VERTEX_SHADER)
      },
      Shader::FragmentShader(ref _name) => unsafe {
        gl::CreateShader(gl::FRAGMENT_SHADER)
      },
    };

    self.load_shader_src(shader, shader_id);

    unsafe { gl::CompileShader(shader_id) }

    let error_result =
      super::gl_error_reader::get_error(GlError::ShaderError(shader_id), 512);

    match error_result {
      GlErrorResult::Error(error) => {
        println!(
          "Shader with id {} could not compile because: {}",
          self.get_shader_location(shader),
          error
        );
      }
      _ => {}
    }

    return shader_id;
  }

  pub fn use_program(&self) {
    unsafe { gl::UseProgram(self.program_id) };
    for (_id, shader) in &self.shader_map {
      match shader {
        Shader::VertexShader(_name, attributes) => {
          for attribute in attributes {
            let attrib_name = get_c_string(attribute.name.to_owned());
            let attrib_location = unsafe {
              let loc =
                gl::GetAttribLocation(self.program_id, attrib_name.as_ptr());
              if loc >= 0 {
                loc as u32
              } else {
                panic!("Location for {:?} not found", attrib_name);
              }
            };

            let gl_normalized = if attribute.normalized {
              gl::TRUE
            } else {
              gl::FALSE
            };

            unsafe {
              match attribute.attrib_type {
                VertexShaderAttributeType::Vector => {
                  gl::VertexAttribPointer(
                    attrib_location,
                    attribute.size,
                    attribute.data_type.to_gl(),
                    gl_normalized,
                    attribute.stride,
                    attribute.offset as *const gl::types::GLvoid,
                  );
                  gl::EnableVertexAttribArray(attrib_location);
                }
                VertexShaderAttributeType::Matrix4 => {
                  for i in 0..4 {
                    let pos = attrib_location + i.to_owned();
                    gl::EnableVertexAttribArray(pos);
                    gl::VertexAttribPointer(
                      pos,
                      4,
                      attribute.data_type.to_gl(),
                      gl_normalized,
                      attribute.stride,
                      ((attribute.offset)
                        + (attribute.data_type.get_size() * (i as i32 * 4)))
                        as *const gl::types::GLvoid,
                    );
                    // gl::VertexAttribDivisor(pos, 1);
                  }
                }
              }
            }
          }
        }
        Shader::FragmentShader(_name) => {}
      }
    }
  }

  pub fn load_shaders(&mut self, shaders: Vec<Shader>) {
    for shader in shaders {
      match shader {
        Shader::VertexShader(ref _name, ref _attributes) => {
          self.shader_map.insert(self.init_shader(&shader), shader);
        }
        Shader::FragmentShader(ref _name) => {
          self.shader_map.insert(self.init_shader(&shader), shader);
        }
      }
    }

    unsafe {
      for (shader_id, _shader) in &self.shader_map {
        gl::AttachShader(self.program_id, shader_id.to_owned())
      }

      gl::LinkProgram(self.program_id);
    }
  }
}

impl Drop for ShaderProgram {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteProgram(self.program_id);

      for (shader_id, _shader) in &self.shader_map {
        gl::DeleteShader(shader_id.to_owned());
      }
    }
  }
}
