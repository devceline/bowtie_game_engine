use crate::math::matrix::Matrix;

use super::gl_translation::{DataType, ToGl};

pub trait SettableUniform<T> {
  fn set_uniform(&self, loc: i32);
  fn get_name(&self) -> String;
}

pub struct UniformFloatVector {
  pub name: String,
  pub count: i8,
  pub values: Vec<f32>,
  data_type: DataType,
}

impl UniformFloatVector {
  pub fn new(name: &str, count: i8, values: Vec<f32>) -> UniformFloatVector {
    UniformFloatVector {
      name: String::from(name),
      count,
      values,
      data_type: DataType::Float32,
    }
  }
}

impl SettableUniform<Vec<f32>> for UniformFloatVector {
  fn get_name(&self) -> String {
    self.name.to_owned()
  }
  fn set_uniform(&self, loc: i32) {
    match self.count {
      3 => {
        unsafe {
          gl::Uniform3f(
            loc,
            self.values[0].into(),
            self.values[1].into(),
            self.values[2].into(),
          );
        };
      }
      _ => {
        panic!(
          "Uniform for vector of {} values not implemented",
          self.count
        )
      }
    }
  }
}

pub struct UniformInteger {
  pub name: String,
  pub value: i32,
  data_type: DataType,
}

impl UniformInteger {
  pub fn new(name: &str, value: i32) -> UniformInteger {
    UniformInteger {
      name: String::from(name),
      value,
      data_type: DataType::Int,
    }
  }
}

impl SettableUniform<i32> for UniformInteger {
  fn get_name(&self) -> String {
    self.name.to_owned()
  }

  fn set_uniform(&self, loc: i32) {
    unsafe {
      gl::Uniform1i(loc, self.value);
    }
  }
}

pub struct UniformMatrixFloat {
  pub name: String,
  pub value: Matrix<f32>,
  data_type: DataType,
}

impl UniformMatrixFloat {
  pub fn new(name: &str, value: Matrix<f32>) -> UniformMatrixFloat {
    UniformMatrixFloat {
      name: String::from(name),
      value,
      data_type: DataType::Float32,
    }
  }
}

impl SettableUniform<Matrix<f32>> for UniformMatrixFloat {
  fn get_name(&self) -> String {
    self.name.to_owned()
  }
  fn set_uniform(&self, loc: i32) {
    unsafe {
      gl::UniformMatrix4fv(loc, 1, gl::TRUE, &self.value.get_inner_ptr()[0])
    }
  }
}
