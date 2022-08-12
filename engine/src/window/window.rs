extern crate gl;
extern crate glfw;

use glfw::Context;

use crate::init_debug_callback;

pub enum WindowMode {
  Windowed,
}

impl WindowMode {
  pub fn to_glfw(&self) -> glfw::WindowMode {
    match self {
      WindowMode::Windowed => glfw::WindowMode::Windowed,
    }
  }
}

pub struct WindowConfig {
  pub width: u16,
  pub height: u16,
  pub name: String,
  pub mode: WindowMode,
}

impl WindowConfig {
  pub fn new(
    width: u16,
    height: u16,
    name: &str,
    mode: WindowMode,
  ) -> WindowConfig {
    WindowConfig {
      width,
      height,
      name: String::from(name),
      mode,
    }
  }

  pub fn create(
    &self,
    glfw_instance: &mut glfw::Glfw,
  ) -> (
    glfw::Window,
    std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
  ) {
    let (mut window, events) = glfw_instance
      .create_window(
        self.width.into(),
        self.height.into(),
        self.name.as_str(),
        self.mode.to_glfw(),
      )
      .expect("Failed to create window");

    window.make_current();

    gl::load_with(|s| glfw_instance.get_proc_address_raw(s));

    // OpenGL 3.2
    glfw_instance.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw_instance.window_hint(glfw::WindowHint::ContextVersionMinor(2));
    glfw_instance.window_hint(glfw::WindowHint::OpenGlProfile(
      glfw::OpenGlProfileHint::Core,
    ));
    glfw_instance.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    glfw_instance.set_swap_interval(glfw::SwapInterval::Sync(1));

    init_debug_callback();

    window.make_current();
    window.set_key_polling(true);
    window.set_sticky_keys(true);

    return (window, events);
  }
}
