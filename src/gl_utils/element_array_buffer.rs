use std::mem::{size_of_val, size_of};

use super::gl_translation::{DataType, DrawingMode, UsageMode, ToGl};

pub struct ElementArrayBuffer<T> {
  pub elements: Vec<T>,
  pub data_type: DataType,
  id: u32
  
}

impl<T> ElementArrayBuffer<T> {
  pub fn new(elements: Vec<T>, data_type: DataType, usage_mode: UsageMode) -> ElementArrayBuffer<T> {
    let mut id = 0;

    unsafe { 
      gl::GenBuffers(1, &mut id);
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
      gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (size_of::<T>() * elements.len()) as isize,
        elements.as_ptr() as *const gl::types::GLvoid,
        usage_mode.to_gl()
      );
    }

    return ElementArrayBuffer {
      elements,
      data_type,
      id
    }

  }

  pub fn draw(&self, mode: DrawingMode) {
    let drawing_mode = mode.to_gl();
    unsafe {
      gl::DrawElements(drawing_mode, self.elements.len() as i32, self.data_type.to_gl(), 0 as *const gl::types::GLvoid);
    }
  }
}

