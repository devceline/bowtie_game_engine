extern crate png;
use std::{fmt::Display, mem};

pub trait ToGl {
  fn to_gl(&self) -> u32;
}

#[derive(Clone, Debug)]
pub enum DataType {
  Float32,
  UnsignedInt,
  Int,
}

impl Copy for DataType {}

impl DataType {
  pub fn get_size(&self) -> i32 {
    match self {
      DataType::Float32 => mem::size_of::<f32>() as i32,
      DataType::UnsignedInt => mem::size_of::<u32>() as i32,
      DataType::Int => mem::size_of::<i32>() as i32,
    }
  }

  pub fn get_data_type_string(&self) -> &str {
    match self {
      DataType::Float32 => "Float32",
      DataType::UnsignedInt => "UnsignedInt",
      DataType::Int => "Int",
    }
  }
}

impl ToGl for DataType {
  fn to_gl(&self) -> u32 {
    match self {
      DataType::Float32 => gl::FLOAT,
      DataType::UnsignedInt => gl::UNSIGNED_INT,
      DataType::Int => gl::INT,
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

#[derive(Copy, Clone, Debug)]
pub enum UsageMode {
  StaticDraw,
}

impl ToGl for UsageMode {
  fn to_gl(&self) -> u32 {
    match self {
      UsageMode::StaticDraw => gl::STATIC_DRAW,
    }
  }
}

impl ToGl for png::ColorType {
  fn to_gl(&self) -> u32 {
    match self {
      png::ColorType::RGBA => gl::RGBA,
      png::ColorType::RGB => gl::RGB,
      _ => panic!("PNG Color format {:?} not supported", self),
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub enum TextureWrap {
  ClampToEdge,
  ClampToBorder,
}

#[derive(Debug, Clone, Copy)]
pub enum TextureFilter {
  Linear,
  LinearMipmap,
}

impl ToGl for TextureWrap {
  fn to_gl(&self) -> u32 {
    match self {
      TextureWrap::ClampToEdge => gl::CLAMP_TO_EDGE,
      TextureWrap::ClampToBorder => gl::CLAMP_TO_BORDER,
    }
  }
}

impl ToGl for TextureFilter {
  fn to_gl(&self) -> u32 {
    match self {
      TextureFilter::Linear => gl::LINEAR,
      TextureFilter::LinearMipmap => gl::LINEAR_MIPMAP_NEAREST,
    }
  }
}
