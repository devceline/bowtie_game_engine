use crate::general::color::Color;

use super::shape::Shape;

#[derive(Debug)]
pub struct Rectangle {
  pub width: f32,
  pub height: f32,
  pub x: f32,
  pub y: f32,
  pub color: Color,
}

const TEX_CORDS_CORNERS: [[f32; 2]; 4] =
  [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

impl Shape for Rectangle {
  fn get_elements(&self) -> Vec<i32> {
    return vec![0, 1, 2, 2, 3, 0];
  }

  fn get_x(&self) -> f32 {
      self.x
  }

  fn get_y(&self) -> f32 {
      self.x
  }

  fn set_x(&mut self, x: f32) {
    self.x = x;
  }

  fn set_y(&mut self, y: f32) {
    self.y = y;
  }

  fn get_corners(&self) -> i32 {
    return 4;
  }

  fn get_vertices(&self) -> Vec<f32> {
    let mut vertices = Vec::<f32>::new();

    let corners = [
      [self.x, self.y],
      [self.x + self.width, self.y],
      [self.x + self.width, self.y - self.height],
      [self.x, self.y - self.height],
    ];

    for i in 0..4 {
      // X, Y
      let [x, y] = corners[i];
      vertices.push(x);
      vertices.push(y);

      // Color
      vertices.push(self.color.r);
      vertices.push(self.color.g);
      vertices.push(self.color.b);
      vertices.push(self.color.a);

      // Texture Cords
      let [tx, ty] = TEX_CORDS_CORNERS[i];
      vertices.push(tx);
      vertices.push(ty);
    }

    return vertices;
  }
}


