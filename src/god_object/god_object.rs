use crate::{
  gl_utils::{
    gl_translation::{DataType, DrawingMode, UsageMode, UsageMode},
    shader_creator::{
      Shader, ShaderProgram, VertexShaderAttribute, VertexShaderAttributeType,
    }, vertex_array_object_handler::VertexArrayObject, vertex_array_object_handler::VertexArrayObject,
  },
  rendering::drawer::Drawer,
  sprites::drawable::Drawable,
};

use super::entity::Entity;

pub struct GodObject<'a> {
  entities: Vec<Box<dyn Entity<'a>>>,
  drawer: Drawer<'a>,
  shading_program: ShaderProgram,
  _vao: VertexArrayObject
}

fn get_program() -> ShaderProgram {
  let mut program = ShaderProgram::new();
  program.load_shaders(vec![
    Shader::VertexShader(
      String::from("main"),
      vec![
        VertexShaderAttribute::new(
          String::from("position"),
          DataType::Float32,
          2,
          9 + (4 * 4),
          true,
          0,
          VertexShaderAttributeType::Vector,
        ),
        VertexShaderAttribute::new(
          String::from("targetColor"),
          DataType::Float32,
          4,
          9 + (4 * 4),
          true,
          2,
          VertexShaderAttributeType::Vector,
        ),
        VertexShaderAttribute::new(
          String::from("tex_cords_in"),
          DataType::Float32,
          2,
          9 + (4 * 4),
          true,
          6,
          VertexShaderAttributeType::Vector,
        ),
        VertexShaderAttribute::new(
          String::from("tex_id"),
          DataType::Float32,
          1,
          9 + (4 * 4),
          true,
          8,
          VertexShaderAttributeType::Vector,
        ),
        VertexShaderAttribute::new(
          String::from("trans"),
          DataType::Float32,
          4,
          9 + (4 * 4),
          true,
          9,
          VertexShaderAttributeType::Matrix4,
        ),
      ],
    ),
    Shader::FragmentShader(String::from("main")),
  ]);
  program
}

impl<'a> GodObject<'a> {
  pub fn new() -> GodObject<'a> {
    let _vao = VertexArrayObject::new();
    GodObject {
      entities: vec![],
      drawer: Drawer::new(UsageMode::StaticDraw),
      shading_program: get_program(),
      _vao 
    }
  }
  pub fn load_entity(&'a mut self, entity: Box<dyn Entity<'a>>) {
    self.entities.push(entity);
    let drawable = self.entities[self.entities.len() - 1].get_drawable();
    self.drawer.load_sprite_dynamic(drawable);
  }

  pub fn update_entities(&mut self) {
    for entity in &self.entities {}
  }

  pub fn prep_for_render(&self) {
    self.shading_program.use_program();
    self.drawer.prep_textures(&self.shading_program);
  }

  pub fn draw_entities(&mut self) {
    self.drawer.draw(DrawingMode::Triangles);
  }
}
