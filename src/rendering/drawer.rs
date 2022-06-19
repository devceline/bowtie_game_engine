use crate::general::color;
use crate::gl_utils::element_array_buffer::ElementArrayBuffer;
use crate::gl_utils::gl_translation::{DataType, DrawingMode, ToGl, UsageMode};
use crate::gl_utils::shader_creator::ShaderProgram;
use crate::gl_utils::vertex_array_buffer::VertexArrayBuffer;
use crate::shapes::rectangle::Rectangle;
use crate::shapes::shape::Shape;
use crate::sprites::drawable::Drawable;

pub struct Drawer<'a> {
  vertex_array_buffer: VertexArrayBuffer<f32>,
  element_array_buffer: ElementArrayBuffer<i32>,
  vertices: Vec<f32>,
  elements: Vec<i32>,
  elements_count: i32,
  dynamic_shapes: Vec<&'a dyn Shape>,
  dynamic_sprites: Vec<&'a dyn Drawable<'a>>,
  shader_program: &'a ShaderProgram,
}

impl<'a> Drawer<'a> {
  pub fn new(
    usage_mode: UsageMode,
    shader_program: &'a ShaderProgram 
  ) -> Drawer<'a> {
    Drawer {
      vertex_array_buffer: VertexArrayBuffer::<f32>::new(
        DataType::Float32,
        usage_mode,
      ),
      element_array_buffer: ElementArrayBuffer::<i32>::new(
        DataType::UnsignedInt,
        usage_mode,
      ),
      shader_program,
      vertices: vec![],
      elements: vec![],
      elements_count: 0,
      dynamic_shapes: vec![],
      dynamic_sprites: vec![]
    }
  }

  fn load_shape(
    elements: &mut Vec<i32>,
    vertices: &mut Vec<f32>,
    shape: &'a dyn Shape,
    elements_count: i32,
  ) {
    unsafe {
      for i in (*shape).get_vertices() {
        vertices.push(i);
      }

      for i in (*shape).get_elements() {
        elements.push(i + elements_count.to_owned());
      }
    }
  }

  fn load_sprite(
    elements: &mut Vec<i32>,
    vertices: &mut Vec<f32>,
    drawable: &'a dyn Drawable<'a>,
    elements_count: i32,
    ) -> () {
      Drawer::load_shape(elements, vertices,  drawable.get_shape_ptr(), elements_count);
  }


  pub fn load_shape_dynamic(&mut self, shape: &'a dyn Shape) {
    self.dynamic_shapes.push(shape);
    Drawer::load_shape(
      &mut self.elements,
      &mut self.vertices,
      shape,
      self.elements_count,
    );
    self.vertex_array_buffer.update_data(&self.vertices);
    self.element_array_buffer.update_data(&self.elements);
  }

  pub fn load_sprite_dynamic(&mut self, sprite: &'a dyn Drawable<'a> )
  {
    self.dynamic_sprites.push(sprite);
    Drawer::load_sprite(
      &mut self.elements,
      &mut self.vertices,
      sprite,
      self.elements_count,
    );
    self.vertex_array_buffer.update_data(&self.vertices);
    self.element_array_buffer.update_data(&self.elements);

    sprite.get_texture_ptr().load_texture(self.shader_program);
  }

  pub fn clear_screen(&mut self, color: color::Color) {
    let clear_rect = Rectangle {
      x: -1.0,
      y: 1.0,
      width: 2.0,
      height: 2.0,
      color,
    };

    self
      .vertex_array_buffer
      .update_data(&clear_rect.get_vertices());
    self
      .element_array_buffer
      .update_data(&clear_rect.get_elements());

    unsafe {
      gl::DrawElements(
        DrawingMode::Triangles.to_gl(),
        self.elements.len() as i32,
        self.element_array_buffer.data_type.to_gl(),
        0 as *const gl::types::GLvoid,
      );
    }
  }

  pub fn draw(&mut self, mode: DrawingMode) {
    let dynamic_shapes = &self.dynamic_shapes;

    self.vertices.clear();
    self.elements.clear();

    for i in 0..dynamic_shapes.len() {
      Drawer::load_shape(
        &mut self.elements,
        &mut self.vertices,
        self.dynamic_shapes[i],
        self.elements_count,
      );
      self.vertex_array_buffer.update_data(&self.vertices);
      self.element_array_buffer.update_data(&self.elements);
    }

    println!("Elements: {:?}", self.elements);
    println!("Vertices: {:?}", self.vertices);
    println!("Shapes: {:?}", self.dynamic_shapes);

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
