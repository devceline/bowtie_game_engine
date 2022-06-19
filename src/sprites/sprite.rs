use crate::{
  gl_utils::{gl_texture::Texture, shader_creator::ShaderProgram},
  shapes::shape::Shape,
};

use super::drawable::Drawable;

pub struct Sprite<'a, TShape>
where
  TShape: Shape + 'a,
{
  shape: TShape,
  _shape_ptr: &'a TShape,
  texture: Texture,
}

impl<'a, TShape: 'a> Sprite<'a, TShape>
where
  TShape: Shape + 'a,
{
  fn new(shape: TShape, texture: Texture) -> Sprite<'a, TShape> {
    let _shape_ptr =
      unsafe { (std::ptr::null() as *const TShape).as_ref().unwrap() };
    Sprite {
      shape,
      texture,
      _shape_ptr,
    }
  }

  pub fn load_texture(&self, program: &ShaderProgram) {
    self.texture.load_texture(program);
  }

  fn move_up(&mut self, amount: f32) -> bool {
    let new_amount = self.shape.get_y() - amount;

    if new_amount < 1.0 {
      return false;
    }

    self.shape.set_y(amount);

    return true;
  }

  fn move_down(&mut self, amount: f32) -> bool {
    let new_amount = self.shape.get_y() + amount;

    if new_amount > 1.0 {
      return false;
    }

    self.shape.set_y(new_amount);

    return true;
  }

  fn move_right(&mut self, amount: f32) -> bool {
    let new_amount = self.shape.get_x() + amount;

    if new_amount > 1.0 {
      return false;
    }

    self.shape.set_x(new_amount);

    return true;
  }

  fn move_left(&mut self, amount: f32) -> bool {
    let new_amount = self.shape.get_x() - amount;

    if new_amount < 1.0 {
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
}
