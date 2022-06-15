extern crate gl;

use std::{collections::HashMap, ffi::CString, fs};

use super::gl_error_reader::{GlError, GlErrorResult};

pub enum GlDataType {
    Float,
    Int,
}
pub struct VertexShaderAttribute {
    pub name: String,
    pub data_type: GlDataType,
    pub size: i32,
    pub stride: i32,
    pub normalized: bool,
    pub pointer: *const std::os::raw::c_void,
}

pub struct Uniform<T> {
    pub name: String,
    pub data_type: GlDataType,
    pub count: i8,
    pub values: Vec<T>,
}

trait TmpTest {
    fn get_name(&self) -> String;
}

pub enum Shader {
    VertexShader(String, Vec<VertexShaderAttribute>),
    FragmentShader(String),
}

impl Shader {
    fn get_name(&self) -> String {
        let thing = 122.0f32 as i32;
        return match self {
            Shader::VertexShader(name, _attributes) => name.to_owned(),
            Shader::FragmentShader(name) => name.to_owned(),
        };
    }
}

fn get_c_string(original_string: String) -> CString {
    return CString::new(original_string).expect("Could not convert to c string");
}

pub struct ShaderProgram {
    program_id: u32,
    shader_map: HashMap<u32, Shader>,
}

impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        unsafe {
            return ShaderProgram {
                program_id: gl::CreateProgram(),
                shader_map: HashMap::new(),
            };
        }
    }

    fn set_uniform<T>(&self, uniform: Uniform<T>)
    where
        T: Into<f32> + Copy,
    {
        let uniform_name = get_c_string(uniform.name);

        let uniform_location =
            unsafe { gl::GetAttribLocation(self.program_id, uniform_name.as_ptr()) };
        match (uniform.count, uniform.data_type) {
            (3, GlDataType::Float) => {
                unsafe {
                    gl::Uniform3f(
                        uniform_location,
                        uniform.values[0].into(),
                        uniform.values[1].into(),
                        uniform.values[2].into(),
                    );
                };
            }
            _ => {}
        }
    }

    fn get_shader_location(&self, shader: &Shader) -> String {
        let mut base_url = String::from("./shaders/");
        match shader {
            Shader::VertexShader(name, _attributes) => {
                base_url.push_str("vertex/");
                base_url.push_str(&name);
            }
            Shader::FragmentShader(name) => {
                base_url.push_str("fragment/");
                base_url.push_str(&name);
            }
        }

        base_url.push_str(".glsl");

        return base_url;
    }

    fn load_shader_src(&self, shader: &Shader, id: u32) {
        let location = self.get_shader_location(shader);
        let source_code =
            fs::read_to_string(location).expect("Could not locate shader at location {}");

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
            Shader::FragmentShader(ref _name) => unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) },
        };

        self.load_shader_src(shader, shader_id);

        unsafe { gl::CompileShader(shader_id) }

        let error_result = super::gl_error_reader::get_error(GlError::ShaderError(shader_id), 512);

        match error_result {
            GlErrorResult::Error(error) => {
                println!(
                    "Shader with id {} could not compile because: {}",
                    shader_id, error
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
                            gl::GetAttribLocation(self.program_id, attrib_name.as_ptr()) as u32
                        };

                        let gl_data_type = match attribute.data_type {
                            GlDataType::Float => gl::FLOAT,
                            GlDataType::Int => gl::UNSIGNED_INT,
                        };

                        let gl_normalized = if attribute.normalized {
                            gl::TRUE
                        } else {
                            gl::FALSE
                        };

                        unsafe {
                            gl::VertexAttribPointer(
                                attrib_location,
                                attribute.size,
                                gl_data_type,
                                gl_normalized,
                                attribute.stride,
                                attribute.pointer,
                            );

                            gl::EnableVertexAttribArray(attrib_location);
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
        println!("Dropping shader program {}", self.program_id);
        unsafe {
            gl::DeleteProgram(self.program_id);

            for (shader_id, _shader) in &self.shader_map {
                gl::DeleteShader(shader_id.to_owned());
            }
        }
    }
}
