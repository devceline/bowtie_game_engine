extern crate gl;

use crate::general::color;
use crate::gl_utils::element_array_buffer::ElementArrayBuffer;
use crate::gl_utils::gl_texture::Texture;
use crate::gl_utils::gl_translation::{DataType, DrawingMode, ToGl, UsageMode};
use crate::gl_utils::shader_creator::ShaderProgram;
use crate::gl_utils::vertex_array_buffer::VertexArrayBuffer;
use crate::shapes::rectangle::Rectangle;
use crate::sprites::drawable::Drawable;
use crate::sprites::sprite::Sprite;

pub struct Drawer<'a> {
  vertex_array_buffer: VertexArrayBuffer<f32>,
  element_array_buffer: ElementArrayBuffer<i32>,
  vertices: Vec<f32>,
  elements: Vec<i32>,
  elements_count: i32,
  dynamic_sprites: Vec<&'a dyn Drawable<'a>>,
  shader_program: &'a ShaderProgram,
}

impl<'a> Drawer<'a> {
  pub fn new(
    usage_mode: UsageMode,
    shader_program: *const ShaderProgram,
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
      shader_program: unsafe { shader_program.as_ref().unwrap() },
      vertices: vec![],
      elements: vec![],
      elements_count: 0,
      dynamic_sprites: vec![],
    }
  }

  /*
   * Loads sprite into this drawer
   * It pushes the sprite's vertices and elements to have it be rendered
   */
  fn load_sprite(
    elements: &mut Vec<i32>,
    vertices: &mut Vec<f32>,
    drawable: &'a dyn Drawable<'a>,
    elements_count: i32,
  ) -> () {
    for i in drawable.get_vertices() {
      vertices.push(i);
    }

    for i in drawable.get_elements() {
      elements.push(i + elements_count.to_owned());
    }
  }

  /*
   * Add sprite to drawer to be rendered on the next draw call.
   * Naturally, the sprite needs to have the same lifetime as the drawer.
   */
  pub fn load_sprite_dynamic(&mut self, sprite: *const dyn Drawable<'a>) {
    unsafe {
      let sprite_instance = sprite.as_ref().unwrap();
      self.dynamic_sprites.push(sprite_instance);
      Drawer::load_sprite(
        &mut self.elements,
        &mut self.vertices,
        sprite_instance,
        self.elements_count,
      );

      self.vertex_array_buffer.update_data(&self.vertices);
      self.element_array_buffer.update_data(&self.elements);

      sprite_instance.load_texture();
    };
  }

  /*
   * Removes sprite from drawer to be removed on the next draw call.
   */
  pub fn unload_sprite_dynamic(&mut self, sprite: *const dyn Drawable<'a>) {
    let to_remove_idx = self
      .dynamic_sprites
      .iter()
      .position(|spr| (*spr as *const dyn Drawable<'a>) == sprite);
    match to_remove_idx {
      Some(idx) => {
        self.dynamic_sprites.remove(idx);
      }
      None => {}
    }
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

  /// Actually loads the sprite's textures.
  /// This needs to be done once, but has to be done before the draw call.
  pub fn prep_textures(&self) {
    for sprite in &self.dynamic_sprites {
      sprite.set_texture_uniform(self.shader_program);
    }
    unsafe {
      gl::Enable(gl::BLEND);
      gl::BlendEquation(gl::FUNC_ADD);
      gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
  }

  /// Renders the dynamically loaded sprites
  pub fn draw(&mut self, mode: DrawingMode) {
    let dynamic_sprites = &self.dynamic_sprites;

    self.vertices.clear();
    self.elements.clear();
    self.elements_count = 0;

    for i in 0..dynamic_sprites.len() {
      Drawer::load_sprite(
        &mut self.elements,
        &mut self.vertices,
        dynamic_sprites[i],
        self.elements_count,
      );
      self.elements_count += dynamic_sprites[i].get_corner_count()
    }

    self.vertex_array_buffer.update_data(&self.vertices);
    self.element_array_buffer.update_data(&self.elements);

    unsafe {
      gl::DrawElementsInstanced(
        mode.to_gl(),
        self.elements.len() as i32,
        self.element_array_buffer.data_type.to_gl(),
        0 as *const gl::types::GLvoid,
        self.dynamic_sprites.len() as i32,
      );
    }
  }
}
