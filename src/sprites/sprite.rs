use crate::{
  gl_utils::{gl_texture::Texture, shader_creator::ShaderProgram},
  shapes::shape::Shape,
};

use std::marker::PhantomData;
use super::drawable::Drawable;

#[derive(Debug)]
pub struct Sprite<'a, TShape>
where
  TShape: Shape + 'a,
{
  shape: TShape,
  texture: Texture,
  phantom: PhantomData<&'a TShape>
}

impl<'a, TShape: 'a> Sprite<'a, TShape>
where
  TShape: Shape + 'a,
{
  pub fn new(shape: TShape, texture: Texture) -> Sprite<'a, TShape> {
    Sprite {
      shape,
      texture,
      phantom: PhantomData
    }
  }

  pub fn load_texture(&self) {
    self.texture.load_texture();
  }

  pub fn set_x(&mut self, x: f32) {
    self.shape.set_x(x);
  }
  pub fn set_y(&mut self, y: f32) {
    self.shape.set_y(y);
  }

  pub fn get_x(&self) -> f32 {
    self.shape.get_x()
  }

  pub fn get_y(&self) -> f32 {
    self.shape.get_y()
  }

  pub fn move_up(&mut self, amount: f32) -> bool {
    let new_amount = self.shape.get_y() + amount;

    if new_amount <= -1.0 {
      return false;
    }

    self.shape.set_y(new_amount);

    return true;
  }

  pub fn move_down(&mut self, amount: f32) -> bool {
    let new_amount = self.shape.get_y() - amount;

    if new_amount >= 1.0 {
      return false;
    }

    self.shape.set_y(new_amount);

    return true;
  }

  const TEX_CORDS_CORNERS: [[f32; 2]; 4] =
    [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

  pub fn move_right(&mut self, amount: f32) -> bool {
    let new_amount = self.shape.get_x() + amount;

    if new_amount > 1.0 {
      return false;
    }

    self.shape.set_x(new_amount);

    return true;
  }

  pub fn move_left(&mut self, amount: f32) -> bool {
    let new_amount = self.shape.get_x() - amount;

    if new_amount <= -1.0 {
      return false;
    }

    self.shape.set_x(new_amount);

    return true;
  }

}

impl<'a, TShape> Drawable<'a> for Sprite<'a, TShape>
where
  TShape: Shape + 'a,
{
  fn get_shape_ptr(&'a self) -> &'a dyn Shape {
    &self.shape
  }

  fn get_texture_ptr(&'a self) -> &'a Texture {
    &self.texture
  }

  fn get_elements(&self) -> Vec<i32> {
    return vec![0, 1, 2, 2, 3, 0];
  }

  fn get_vertices(&self) -> Vec<f32> {

    let mut vertices = Vec::<f32>::new();

    let shape = &self.shape;

    let corners = [
      [shape.get_x(), shape.get_y()],
      [shape.get_x() + shape.get_width(), shape.get_y()],
      [shape.get_x() + shape.get_width(), shape.get_y() - shape.get_height()],
      [shape.get_x(), shape.get_y() - shape.get_height()],
    ];

    for i in 0..4 {
      // X, Y
      let [x, y] = corners[i];
      vertices.push(x);
      vertices.push(y);

      // Color
      vertices.push(shape.get_color().r);
      vertices.push(shape.get_color().g);
      vertices.push(shape.get_color().b);
      vertices.push(shape.get_color().a);

      // Texture Cords
      let [tx, ty] = Sprite::<TShape>::TEX_CORDS_CORNERS[i];
      vertices.push(tx);
      vertices.push(ty);

      vertices.push(self.texture.texture_id as f32);

    }
      return vertices;
  }

}
