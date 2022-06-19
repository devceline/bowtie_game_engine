use crate::{general::color::Color, gl_utils::gl_texture::Texture};

use super::shape::Shape;

#[derive(Debug)]
pub struct Rectangle {
  pub width: f32,
  pub height: f32,
  pub x: f32,
  pub y: f32,
  // texture: Texture,
  pub color: Color,
}

const TEX_CORDS_CORNERS: [[f32; 2]; 4] =
  [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

impl Shape for Rectangle {
  fn get_elements(&self) -> Vec<i32> {
    return vec![0, 1, 2, 2, 3, 0];
  }

  fn get_corners(&self) -> i32 {
    return 4;
  }

  fn move_up(&mut self, amount: f32) -> bool {
      let new_amount = self.y - amount;

      if new_amount < 1.0 {
        return false;
      }

      self.y = new_amount;

      return true;
  }

  fn move_down(&mut self, amount: f32) -> bool {
      let new_amount = self.y + amount;

      if new_amount > 1.0 {
        return false;
      }

      self.y = new_amount;

      return true;
  }

  fn move_right(&mut self, amount: f32) -> bool {
      let new_amount = self.x + amount;

      if new_amount > 1.0 {
        return false;
      }

      self.x = new_amount;

      return true;
  }

  fn move_left(&mut self, amount: f32) -> bool {
      let new_amount = self.x - amount;

      if new_amount < 1.0 {
        return false;
      }

      self.x = new_amount;

      return true;
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


