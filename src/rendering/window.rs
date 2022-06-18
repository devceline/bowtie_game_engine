use crate::shapes::shape::Shape;

pub struct Window {
  width: f32,
  height: f32
}

impl Window {
  pub fn new(width: f32, height: f32) -> Window {
    Window { width, height }
  }

  pub fn draw<TShape>(&self, shape: TShape) where TShape: Shape {

  }

  pub fn init(&mut self) {
  }
}


