use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

extern crate gl;
extern crate glfw;

use glfw::Context;

use crate::{
  general::color::COLORS,
  gl_utils::{
    gl_translation::{DataType, DrawingMode, UsageMode},
    shader_creator::{
      Shader, ShaderProgram, VertexShaderAttribute, VertexShaderAttributeType,
    },
  },
  init_debug_callback,
  rendering::drawer::Drawer,
  window::window::WindowConfig,
};

use super::entity::StandardEntity;

/// Public interface for the game engine's capabilities
/// Will be responsible for rendering, handling physics systems
/// And controlling the game's state through entitiy data
pub struct BowTie {
  entities: Vec<Option<Rc<RefCell<StandardEntity>>>>,
  free_entity_slots: VecDeque<usize>,
  drawer: Drawer,
  shading_program: ShaderProgram,
  glfw_instance: glfw::Glfw,
  window: Option<glfw::Window>,
  events: Option<std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>>,
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

impl BowTie {
  pub fn new() -> BowTie {
    let mut bowtie = BowTie {
      entities: vec![],
      free_entity_slots: VecDeque::from([]),
      drawer: Drawer::shell(),
      shading_program: ShaderProgram::shell(),
      glfw_instance: glfw::init(glfw::FAIL_ON_ERRORS).unwrap(),
      window: None,
      events: None,
    };
    bowtie.drawer.set_entities_array(&bowtie.entities);
    bowtie
  }

  /// Loads the entity into the drawer and the game's state
  /// To handle rendering and physics
  pub fn load_entity(&mut self, entity: Rc<RefCell<StandardEntity>>) -> usize {
    let free_idx = self.free_entity_slots.pop_front();
    match free_idx {
      Option::Some(idx) => {
        self.entities[idx] = Option::from(entity);
        return idx;
      }
      Option::None => {
        self.entities.push(Option::from(entity));
        return self.entities.len() - 1;
      }
    }
  }

  pub fn unload_entity(&mut self, entity_id: usize) {
    self.entities[entity_id] = Option::None;
    self.free_entity_slots.push_back(entity_id);
  }

  pub fn unload_entities_out_of_view(&mut self) {
      let entities_count = self.entities.len();
      for entity_idx in  0..entities_count {
          if let Some(entity) = &self.entities[entity_idx] {
              let entity_is_out_of_horizontal_view = entity.borrow().sprite.get_x() > 1.0 || entity.borrow().sprite.get_x() < -1.0;
              let entity_is_out_of_vertical_view = entity.borrow().sprite.get_y() > 1.0 || entity.borrow().sprite.get_y() < -1.0;
              if  entity_is_out_of_vertical_view || entity_is_out_of_horizontal_view {
                self.unload_entity(entity_idx);
              }
          }
      }
  }

  pub fn get_entity_count(&self) -> usize {
    self.entities.len() - self.free_entity_slots.len()
  }

  /// Updates the entities with the existing systems
  pub fn update_entities(&mut self) {
    for entity_option in self.entities.iter_mut() {
      if let Some(entity) = entity_option {
        entity.borrow_mut().act_on_components();
      }
    }
  }

  /// Prepares the god object to draw stuff.
  /// Has to be called before the main draw call
  pub fn prep_for_render(&mut self) {
    if self.entities.len() < 1 {
      // self.load_entity(StandardEntity::new(
      //   Sprite::new(
      //     Rectangle::new(0.0, 0.0, 0.0, 0.0, COLORS::White.into()),
      //     Texture::none(),
      //   ),
      //   0.0,
      // ));
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

  pub fn create_window(&mut self, window_config: WindowConfig) {
    let (mut window, events) = self
      .glfw_instance
      .create_window(
        window_config.width.into(),
        window_config.height.into(),
        window_config.name.as_str(),
        window_config.mode.to_glfw(),
      )
      .expect("Failed to create window");

    if window_config.monitor_idx != 0 {
      self.glfw_instance.with_connected_monitors(|_, monitors| {
        if window_config.monitor_idx > monitors.len() - 1 {
          panic!("Supplied monitor not connected.")
        }
        let selected_monitor = &monitors[window_config.monitor_idx];
        let (monitor_x, monitor_y) = selected_monitor.get_pos();
        println!(
          "Setting new position to {}, {}\nUsing monitor {}",
          monitor_x, monitor_y, 0
        );

        window.set_pos(monitor_x, monitor_y);
      });
    }

    window.make_current();

    gl::load_with(|s| self.glfw_instance.get_proc_address_raw(s));

    // OpenGL 3.2
    self
      .glfw_instance
      .window_hint(glfw::WindowHint::ContextVersionMajor(3));
    self
      .glfw_instance
      .window_hint(glfw::WindowHint::ContextVersionMinor(2));
    self
      .glfw_instance
      .window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
      ));
    self
      .glfw_instance
      .window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    self
      .glfw_instance
      .set_swap_interval(glfw::SwapInterval::Sync(1));

    init_debug_callback();

    window.make_current();
    window.set_key_polling(true);
    window.set_sticky_keys(true);

    self.shading_program = get_program();
    self.drawer = Drawer::new(UsageMode::StaticDraw);
    self.drawer.set_entities_array(&self.entities);

    self.window = Option::Some(window);
    self.events = Option::Some(events);
  }

  pub fn flush_events(&self) -> Vec<glfw::WindowEvent> {
    let events = glfw::flush_messages(self.events.as_ref().unwrap());
    let mut window_events: Vec<glfw::WindowEvent> = vec![];
    for (_, event) in events {
      window_events.push(event);
    }
    window_events
  }

  pub fn tick(&mut self) {
    self.unload_entities_out_of_view();
    self.window.as_mut().unwrap().swap_buffers();
    self.glfw_instance.poll_events();
    self.update_entities();
    self.draw_entities();
  }

  pub fn should_close(&self) -> bool {
    self.window.as_ref().unwrap().should_close()
  }

  pub fn set_should_close(&mut self, should: bool) {
    self.window.as_mut().unwrap().set_should_close(should)
  }

}
