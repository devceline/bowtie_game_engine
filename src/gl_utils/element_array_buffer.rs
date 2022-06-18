use std::mem::{size_of, size_of_val};

use super::gl_translation::{DataType, DrawingMode, ToGl, UsageMode};

pub struct ElementArrayBuffer<T> {
  pub data_type: DataType,
  elements: Vec<T>,
  pub usage_mode: UsageMode,
  id: u32,
}

impl<T> ElementArrayBuffer<T> {
  /**
   * Generates a gl element buffer, binds and loads data from elements.
   * Then, an ElementArrayBuffer with the buffer id is returned.
   */
  pub fn new(
    data_type: DataType,
    usage_mode: UsageMode,
  ) -> ElementArrayBuffer<T> {
    let mut id = 0;

    unsafe {
      gl::GenBuffers(1, &mut id);
    }

    return ElementArrayBuffer {
      data_type,
      elements: vec![],
      usage_mode,
      id,
    };
  }

  pub fn update_data(&self, elements: &Vec<T>) {
    unsafe {
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
      gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (size_of::<T>() * elements.len()) as isize,
        elements.as_ptr() as *const gl::types::GLvoid,
        self.usage_mode.to_gl(),
      );
    }
  }

  pub fn draw(&self, mode: DrawingMode) {
    let drawing_mode = mode.to_gl();
    unsafe {
      gl::DrawElements(
        drawing_mode,
        self.elements.len() as i32,
        self.data_type.to_gl(),
        0 as *const gl::types::GLvoid,
      );
    }
  }
}

impl<T> Drop for ElementArrayBuffer<T> {
  fn drop(&mut self) {
    unsafe { gl::DeleteBuffers(1, &self.id) };
  }
}
