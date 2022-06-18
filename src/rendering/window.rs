extern crate gl;
extern crate glfw;
use std::sync::mpsc::Receiver;

use glfw::Context;

use crate::gl_utils::gl_error_reader;
use crate::shapes::shape::Shape;

pub enum WindowMode {
  Windowed,
}

impl WindowMode {
  fn to_glfw(&self) -> glfw::WindowMode {
    match self {
      WindowMode::Windowed => glfw::WindowMode::Windowed,
    }
  }
}


pub struct Window {
  width: u32,
  height: u32,
  should_close: bool,
  _glfw_window: glfw::Window,
  _glfw: glfw::Glfw,
  pub event_reciever: Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {

  pub fn listen_for_errors(&self) {
    gl_error_reader::init_debug_callback();
  }

  pub fn should_close(&self) -> bool {
    self.should_close
  }

  pub fn new(width: u32, height: u32, title: &str, mode: WindowMode) -> Window {
    let mut glfw_instance = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw_instance
      .create_window(width, height, title, mode.to_glfw())
      .expect("Failed to create window");

    Window::window_setup(&mut glfw_instance, &mut window);

    return Window {
      width,
      height,
      should_close: false,
      _glfw: glfw_instance,
      _glfw_window: window,
      event_reciever: events,
    };
  }


  pub fn set_should_close(&mut self, should_close: bool) {
    self.should_close = should_close;
  }

  pub fn close(&mut self) {
    unsafe {
      self._glfw_window.close();
    }
  }

  pub fn draw<TShape>(&self, shape: TShape)
  where
    TShape: Shape,
  {
  }

  #[inline(always)]
  pub fn get_events(
    event_reciever: &Receiver<(f64, glfw::WindowEvent)>,
  ) -> glfw::FlushedMessages<(f64, glfw::WindowEvent)> {
    glfw::flush_messages(&event_reciever).into()
  }

  pub fn swap_buffers(&mut self) {
    self._glfw_window.swap_buffers()
  }

  pub fn poll_events(&mut self) {
    self._glfw.poll_events()
  }
}

impl Drop for Window {
  fn drop(&mut self) {
  }
}


