use std::mem::{size_of };

use super::gl_translation::{DataType, ToGl, UsageMode, DrawingMode};

pub struct VertexArrayBuffer<T> {
  id: u32,
  data_type: DataType,
  vertices: Vec<T>,
}

impl<T> VertexArrayBuffer<T> {
  /**
   * Generates a gl vertex array buffer, binds and loads data from elements. 
   * Then, a VertexArrayBuffer with the buffer id is returned.
   */
  pub fn new(vertices: Vec<T>, data_type: DataType, usage_mode: UsageMode) -> VertexArrayBuffer<T> {
    let mut id: u32 = 0;
    unsafe {
      gl::GenBuffers(1, &mut id);
      gl::BindBuffer(gl::ARRAY_BUFFER, id);
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (size_of::<T>() * vertices.len()) as isize,
        vertices.as_ptr() as *const gl::types::GLvoid,
        usage_mode.to_gl(),
      );
    }

    return VertexArrayBuffer {
      id,
      vertices,
      data_type,
    };
  }

  pub fn draw(&self, drawing_mode: DrawingMode) {
    unsafe {
      gl::DrawArrays(drawing_mode.to_gl(), 0, self.vertices.len() as i32);
    }
  }
}

impl<T> Drop for VertexArrayBuffer<T> {
  fn drop(&mut self) {
    unsafe { gl::DeleteBuffers(1, &self.id) };
  }
}
