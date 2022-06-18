use crate::gl_utils::element_array_buffer::ElementArrayBuffer;
use crate::gl_utils::gl_translation::{UsageMode, DataType, DrawingMode, ToGl};
use crate::shapes::shape::Shape;
use crate::gl_utils::vertex_array_buffer::VertexArrayBuffer;

pub struct Drawer {
  vertex_array_buffer: VertexArrayBuffer<f32>,
  element_array_buffer: ElementArrayBuffer<i32>,
  vertices: Vec<f32>,
  elements: Vec<i32>,
  elements_count: i32,
}

impl Drawer {
  pub fn new(usage_mode: UsageMode) -> Drawer {
    Drawer {
      vertex_array_buffer: VertexArrayBuffer::<f32>::new(DataType::Float32, usage_mode),
      element_array_buffer: ElementArrayBuffer::<i32>::new(DataType::UnsignedInt, usage_mode),
      vertices: vec![],
      elements: vec![],
      elements_count: 0
    }
  }
}

impl Drawer {
  pub fn load_shape<TShape>(&mut self, shape: TShape) where TShape: Shape {

    for i in shape.get_vertices() {
      self.vertices.push(i);
    }

    for i in shape.get_elements() {
      self.elements.push(i + self.elements_count);
    }

    self.vertex_array_buffer.update_data(&self.vertices);
    self.element_array_buffer.update_data(&self.elements);
  }

  pub fn draw(&self, mode: DrawingMode) {
    unsafe {
      gl::DrawElements(
        mode.to_gl(),
        self.elements.len() as i32,
        self.element_array_buffer.data_type.to_gl(),
        0 as *const gl::types::GLvoid,
      );
    }
  }
}
