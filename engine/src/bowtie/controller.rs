use crate::{
  general::color::COLORS,
  gl_utils::{
    gl_texture::{LoadableTexture, Texture},
    gl_translation::{DataType, DrawingMode, UsageMode},
    shader_creator::{
      Shader, ShaderProgram, VertexShaderAttribute, VertexShaderAttributeType,
    },
    vertex_array_object_handler::VertexArrayObject,
  },
  rendering::drawer::{DrawableData, Drawer},
  Rectangle, Sprite,
};

use super::entity::{Entity, StandardEntity};

/// Public interface for the game engine's capabilities
/// Will be responsible for rendering, handling physics systems
/// And controlling the game's state through entitiy data
pub struct BowTie<'d> {
  entities: Vec<StandardEntity<'d>>,
  drawer: Drawer<'d>,
  shading_program: ShaderProgram,
  _vao: VertexArrayObject,
}

/// Initiates a shader program with pre-defined vertex attributes
// TODO: This needs to be more flexible
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

impl<'d> BowTie<'d> {
  pub fn new() -> BowTie<'d> {
    let _vao = VertexArrayObject::new();
    let mut bowtie = BowTie {
      entities: vec![],
      drawer: Drawer::new(UsageMode::StaticDraw),
      shading_program: get_program(),
      _vao,
    };
    bowtie.drawer.set_entities_array(&bowtie.entities);
    bowtie
  }

  /// Loads the entity into the drawer and the game's state
  /// To handle rendering and physics
  pub fn load_entity(
    &mut self,
    entity: StandardEntity<'d>,
  ) -> &mut StandardEntity<'d> {
    self.entities.push(entity);
    let entity_id = self.entities.len() - 1;
    &mut self.entities[entity_id]
  }

  pub fn unload_entity(&mut self, entity: StandardEntity<'d>) {}

  pub fn get_entity_count(&self) -> usize {
    self.entities.len()
  }

  /// Updates the entities with the existing systems
  pub fn update_entities(&mut self) {
    for entity in self.entities.iter_mut() {
      entity.act_on_components();
    }
  }

  /// Prepares the god object to draw stuff.
  /// Has to be called before the main draw call
  pub fn prep_for_render(&mut self) {
    if self.entities.len() < 1 {
      self.load_entity(StandardEntity::new(
        Sprite::new(
          Rectangle::new(0.0, 0.0, 0.0, 0.0, COLORS::White.into()),
          Texture::none(),
        ),
        0.0,
      ));
    }
    self.drawer.prep_data(&self.shading_program);
    self.shading_program.use_program();
    self.drawer.prep_textures(&self.shading_program);
  }

  /// Draws the entities with an actual clear screen refresh
  pub fn draw_entities(&mut self) {
    self.drawer.clear_screen(COLORS::White.into());
    self
      .drawer
      .draw(DrawingMode::Triangles, &self.shading_program);
  }
}
