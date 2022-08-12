extern crate gl;

use crate::general::color;
use crate::gl_utils::element_array_buffer::ElementArrayBuffer;
use crate::gl_utils::gl_texture::{LoadableTexture, Texture};
use crate::gl_utils::gl_texture_loader::TextureLoader;
use crate::gl_utils::gl_translation::{DataType, DrawingMode, ToGl, UsageMode};
use crate::gl_utils::shader_creator::ShaderProgram;
use crate::gl_utils::vertex_array_buffer::VertexArrayBuffer;
use crate::shapes::rectangle::Rectangle;
use crate::sprites::drawable::Drawable;
use crate::sprites::sprite::Sprite;
use crate::{Entity, StandardEntity};

#[derive(Clone)]
pub struct DrawableData {
  pub vertices: Vec<f32>,
  pub elements: Vec<i32>,
  pub corner_count: i32,
  pub texture: Texture,
}

pub struct Drawer<'a> {
  vertex_array_buffer: VertexArrayBuffer<f32>,
  element_array_buffer: ElementArrayBuffer<i32>,
  vertices: Vec<f32>,
  elements: Vec<i32>,
  texture_loader: TextureLoader,
  elements_count: i32,
  drawables: Vec<DrawableData>,
  entities: *const Vec<StandardEntity<'a>>,
}

impl<'a> Drawer<'a> {
  pub fn shell() -> Drawer<'a> {
    Drawer {
      vertex_array_buffer: VertexArrayBuffer::shell(),
      element_array_buffer: ElementArrayBuffer::shell(),
      texture_loader: TextureLoader::new(),
      vertices: vec![],
      elements: vec![],
      elements_count: 0,
      drawables: vec![],
      entities: std::ptr::null(),
    }
  }
  pub fn new(usage_mode: UsageMode) -> Drawer<'a> {
    Drawer {
      vertex_array_buffer: VertexArrayBuffer::<f32>::new(
        DataType::Float32,
        usage_mode,
      ),
      element_array_buffer: ElementArrayBuffer::<i32>::new(
        DataType::UnsignedInt,
        usage_mode,
      ),
      texture_loader: TextureLoader::new(),
      vertices: vec![],
      elements: vec![],
      elements_count: 0,
      drawables: vec![],
      entities: std::ptr::null(),
    }
  }

  /*
   * Loads sprite into this drawer
   * It pushes the sprite's vertices and elements to have it be rendered
   */

  fn load_drawable(
    elements: &mut Vec<i32>,
    vertices: &mut Vec<f32>,
    drawable: &DrawableData,
    elements_count: i32,
  ) {
    for i in &drawable.vertices {
      vertices.push(i.to_owned());
    }

    for i in &drawable.elements {
      elements.push(i + elements_count.to_owned());
    }
  }

  pub fn load_drawable_dynamic(
    &mut self,
    drawable: DrawableData,
    program: &ShaderProgram,
  ) {
    self.drawables.push(drawable.to_owned());

    Drawer::load_drawable(
      &mut self.elements,
      &mut self.vertices,
      &self.drawables[self.drawables.len() - 1],
      self.elements_count,
    );

    self.vertex_array_buffer.update_data(&self.vertices);
    self.element_array_buffer.update_data(&self.elements);
  }

  /*
   * Renders a rectangle as wide and tall as the window to clear it
   */
  pub fn clear_screen(&mut self, color: color::Color) {
    let clear_rect =
      Sprite::new(Rectangle::new(-1.0, 1.0, 2.0, 2.0, color), Texture::none());

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

  pub fn prep_data(&mut self, program: &ShaderProgram) {
    self.load_all(program);
  }

  /// Actually loads the sprite's textures.
  /// This needs to be done once, but has to be done before the draw call.
  pub fn prep_textures(&mut self, program: &ShaderProgram) {
    let entities = unsafe { self.entities.as_ref().unwrap() };
    let textures = entities
      .iter()
      .map(|entitiy| entitiy.get_drawable().texture.to_owned())
      .collect::<Vec<Texture>>();

    self.texture_loader.load_textures(textures, program);

    unsafe {
      gl::Enable(gl::BLEND);
      gl::BlendEquation(gl::FUNC_ADD);
      gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
  }

  pub fn set_entities_array(
    &mut self,
    entities: *const Vec<StandardEntity<'a>>,
  ) {
    self.entities = entities;
  }

  pub fn load_all(&mut self, program: &ShaderProgram) {
    let size = self.vertices.len();
    self.vertices.clear();
    self.elements.clear();
    self.elements_count = 0;

    self.vertices.reserve(size);

    let entities = unsafe { self.entities.as_ref().unwrap() };
    let len = entities.len();

    for i in 0..len {
      let entity = &entities[i];

      let drawable = entity.get_drawable();
      self
        .texture_loader
        .load_texture(drawable.to_owned().texture, program);
      Drawer::load_drawable(
        &mut self.elements,
        &mut self.vertices,
        &drawable,
        self.elements_count,
      );
      self.elements_count += drawable.corner_count
    }

    self.vertex_array_buffer.update_data(&self.vertices);
    self.element_array_buffer.update_data(&self.elements);
  }

  /// Renders the dynamically loaded sprites
  pub fn draw(&mut self, mode: DrawingMode, program: &ShaderProgram) {
    self.load_all(program);

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
