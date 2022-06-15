use std::mem;

pub trait ToGl {
  fn to_gl(&self) -> u32;
}

pub enum DataType {
  Float32,
  UnsignedInt,
}

impl Clone for DataType {
  fn clone(&self) -> Self {
    match self {
      DataType::Float32 => DataType::Float32,
      DataType::UnsignedInt => DataType::UnsignedInt
    }
  }
}

impl Copy for DataType {}

impl DataType {
  pub fn get_size(&self) -> i32 {
    match self {
      DataType::Float32 => mem::size_of::<f32>() as i32,
      DataType::UnsignedInt => mem::size_of::<u32>() as i32,
    }
  }
}

impl ToGl for DataType {
  fn to_gl(&self) -> u32 {
    match self {
      DataType::Float32 => gl::FLOAT,
      DataType::UnsignedInt => gl::UNSIGNED_INT,
    }
  }
}

pub enum DrawingMode {
  Triangles,
}

impl ToGl for DrawingMode {
  fn to_gl(&self) -> u32 {
    match self {
      DrawingMode::Triangles => gl::TRIANGLES,
    }
  }
}

pub enum UsageMode {
  StaticDraw
}

impl ToGl for UsageMode {
  fn to_gl(&self) -> u32 {
    match self {
      UsageMode::StaticDraw => gl::STATIC_DRAW
    }
  }
}
