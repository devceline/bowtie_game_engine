use crate::{
  general::color::COLORS,
  gl_utils::{
    gl_translation::{DataType, DrawingMode, UsageMode},
    shader_creator::{
      Shader, ShaderProgram, VertexShaderAttribute, VertexShaderAttributeType,
    },
    vertex_array_object_handler::VertexArrayObject,
  },
  rendering::drawer::Drawer,
};

use super::entity::Entity;

/// Public interface for the game engine's capabilities
/// Will be responsible for rendering, handling physics systems
/// And controlling the game's state through entitiy data
pub struct BowTie<'d> {
  entities: Vec<*mut dyn Entity<'d>>,
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
    BowTie {
      entities: vec![],
      drawer: Drawer::new(UsageMode::StaticDraw),
      shading_program: get_program(),
      _vao,
    }
  }

  /// Loads the entity into the drawer and the game's state
  /// To handle rendering and physics
  pub fn load_entity<'g>(&'g mut self, entity: *mut dyn Entity<'d>) {
    self.entities.push(entity);
    unsafe {
      let drawable = (*self.entities[self.entities.len() - 1]).get_drawable();
      self.drawer.load_sprite_dynamic(drawable);
    }
  }

  pub fn unload_entity<'g>(&'g mut self, entity: *mut dyn Entity<'d>) {
    let pos_option = self.entities.iter().position(|entity_ref| entity_ref.to_owned() == entity.to_owned());

    match pos_option {
      Some(pos) => {
        unsafe {
          let drawable = (*self.entities[pos]).get_drawable();
          self.drawer.unload_sprite_dynamic(drawable);
        }
        self.entities.remove(pos);
      }
      None => {}
    }

  }

  /// Updates the entities with the existing systems
  pub fn update_entities(&mut self) {
    for entity in self.entities.to_owned() {
      unsafe {
        for comp in entity.as_mut().unwrap().get_components() {
          let mut entities_copy = self.entities.to_owned();
          let entity_clown = entity.to_owned();
          let message =
            comp.as_mut().unwrap().act(&mut entities_copy, entity_clown);
          match message {
            Some(m) => entity.as_mut().unwrap().recieve_message(m),
            None => {}
          }
        }
      }
    }
  }

  /// Prepares the god object to draw stuff.
  /// Has to be called before the main draw call
  pub fn prep_for_render(&self) {
    self.shading_program.use_program();
    self.drawer.prep_textures(&self.shading_program);
  }

  /// Draws the entities with an actual clear screen refresh
  pub fn draw_entities(&mut self) {
    self.drawer.clear_screen(COLORS::White.into());
    self.drawer.draw(DrawingMode::Triangles);
  }
}
