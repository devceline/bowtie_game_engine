use std::mem::size_of;

use super::gl_translation::{DataType, DrawingMode, ToGl, UsageMode};

pub struct VertexArrayBuffer<T> {
  id: u32,
  data_type: DataType,
  vertices: Vec<T>,
  usage_mode: UsageMode,
}

impl<T> VertexArrayBuffer<T> {
  pub fn shell() -> VertexArrayBuffer<T> {
    VertexArrayBuffer {
      id: 0,
      data_type: DataType::Float32,
      vertices: vec![],
      usage_mode: UsageMode::StaticDraw,
    }
  }
  /**
   * Generates a gl vertex array buffer, binds and loads data from elements.
   * Then, a VertexArrayBuffer with the buffer id is returned.
   */
  pub fn new(
    data_type: DataType,
    usage_mode: UsageMode,
  ) -> VertexArrayBuffer<T> {
    let mut id: u32 = 0;
    unsafe {
      gl::GenBuffers(1, &mut id);
    }

    return VertexArrayBuffer {
      id,
      data_type,
      vertices: vec![],
      usage_mode,
    };
  }

  pub fn get_vertices_len(&self) -> usize {
    self.vertices.len()
  }

  pub fn update_data(&mut self, vertices: &Vec<T>) {
    unsafe {
      gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (size_of::<T>() * vertices.len()) as isize,
        vertices.as_ptr() as *const gl::types::GLvoid,
        self.usage_mode.to_gl(),
      )
    }
  }

  pub fn draw(&self, drawing_mode: DrawingMode) {
    unsafe {
      gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
      gl::DrawArrays(drawing_mode.to_gl(), 0, self.vertices.len() as i32);
    }
  }
}

impl<T> Drop for VertexArrayBuffer<T> {
  fn drop(&mut self) {
    unsafe { gl::DeleteBuffers(1, &self.id) };
  }
}
