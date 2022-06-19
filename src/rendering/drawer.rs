extern crate gl;

use crate::general::color;
use crate::gl_utils::element_array_buffer::ElementArrayBuffer;
use crate::gl_utils::gl_texture::Texture;
use crate::gl_utils::gl_translation::{DataType, DrawingMode, ToGl, UsageMode};
use crate::gl_utils::shader_creator::ShaderProgram;
use crate::gl_utils::vertex_array_buffer::VertexArrayBuffer;
use crate::shapes::rectangle::Rectangle;
use crate::shapes::shape::Shape;
use crate::sprites::drawable::Drawable;
use crate::sprites::sprite::Sprite;

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
      dynamic_shapes: vec![],
      dynamic_sprites: vec![],
    }
  }

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

      sprite_instance.get_texture_ptr().load_texture();
    };
  }

  pub fn unload_sprite_dynamic(&mut self, sprite: *const dyn Drawable<'a>) {
    let to_remove_idx = self.dynamic_sprites.iter().position(|spr| (*spr as *const dyn Drawable<'a>) == sprite).unwrap();
    self.dynamic_sprites.remove(to_remove_idx);
  }

  pub fn clear_screen(&mut self, color: color::Color) {
    let clear_rect = Sprite::new(Rectangle {
      x: -1.0,
      y: 1.0,
      width: 2.0,
      height: 2.0,
      color,
    }, Texture::none());

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

  pub fn prep_textures(&self) {
    for sprite in &self.dynamic_sprites {
      sprite.get_texture_ptr().set_uniform(self.shader_program);
    }
    unsafe {
      gl::Enable(gl::BLEND);
      gl::BlendEquation(gl::FUNC_ADD);
      gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
  }

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
      self.elements_count += dynamic_sprites[i].get_shape_ptr().get_corners();
    }

    self.vertex_array_buffer.update_data(&self.vertices);
    self.element_array_buffer.update_data(&self.elements);

    // println!("Elements: {:?}", self.elements);
    // println!("Vertices: {:?}", self.vertices);
    // println!("Shapes: {:?}", self.dynamic_shapes);
    // println!("Sprites: {:?}", self.dynamic_sprites);

    // println!("Column:  X   Y      R     G   B   A   T_X   T_Y   T_I");
    // println!("Tex id:{:?}", &self.dynamic_sprites[0].get_vertices()[0..9]);
    // println!("Tex id:{:?}", &self.dynamic_sprites[1].get_vertices()[0..9]);


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
