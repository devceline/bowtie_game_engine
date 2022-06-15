use std::mem::{size_of_val, size_of};

use super::gl_translation::{DataType, UsageMode, ToGl};

pub struct VertexArrayBuffer<T> {
  id: u32,
  data_type: DataType,
  vertices: Vec<T>
}

impl<T> VertexArrayBuffer<T> {
  pub fn new(vertices: Vec<T>, data_type: DataType, usage_mode: UsageMode) -> VertexArrayBuffer<T> {
    let mut id: u32 = 0;
    unsafe {
      gl::GenBuffers(1, &mut id);
      gl::BindBuffer(gl::ARRAY_BUFFER, id);
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (size_of::<T>() * vertices.len()) as isize,
        vertices.as_ptr() as *const gl::types::GLvoid,
        usage_mode.to_gl()
      );
    }

    return VertexArrayBuffer {
      id,
      vertices,
      data_type
    }
  }
}
