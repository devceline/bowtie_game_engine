use crate::{
  general::{color::Color, direction::Direction},
  gl_utils::{
    gl_texture::LoadableTexture, gl_texture::Texture,
    shader_creator::ShaderProgram,
  },
  math::matrix::{IdentityMatrix, Matrix},
  shapes::shape::Shape,
};

use super::drawable::Drawable;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Sprite<'a, TShape>
where
  TShape: Shape + 'a,
{
  shape: TShape,
  direction: Direction,
  pub name: String,
  texture: Texture,
  phantom: PhantomData<&'a TShape>,
  transformation: Matrix<f32>,
}

impl<'a, TShape: 'a> Sprite<'a, TShape>
where
  TShape: Shape + 'a,
{
  pub fn new(shape: TShape, texture: Texture) -> Sprite<'a, TShape> {
    Sprite {
      shape,
      name: texture.image_name.to_owned(),
      direction: Direction::Stationary,
      texture,
      phantom: PhantomData,
      transformation: Matrix::<f32>::generate_identity(4),
    }
  }

  pub fn with_transformation(
    shape: TShape,
    texture: Texture,
    trans: Matrix<f32>,
  ) -> Sprite<'a, TShape> {
    Sprite {
      shape,
      direction: Direction::Stationary,
      name: texture.image_name.to_owned(),
      texture,
      phantom: PhantomData,
      transformation: trans,
    }
  }

  fn handle_direction_change(
    &mut self,
    direction: Direction,
    event: &glfw::WindowEvent,
  ) {
    match event {
      glfw::WindowEvent::Key(_, _, glfw::Action::Release, _) => {
        self.direction = self.direction.subtract_direction(direction);
      }
      _ => {
        self.direction = self.direction.add_direction(direction);
      }
    }
  }

  pub async fn respond_to_event(&mut self, event: &glfw::WindowEvent) {
    match event {
      glfw::WindowEvent::Key(glfw::Key::Right, _, _, _) => {
        self.handle_direction_change(Direction::Right, event);
      }
      glfw::WindowEvent::Key(glfw::Key::Left, _, _, _) => {
        self.handle_direction_change(Direction::Left, event);
      }
      glfw::WindowEvent::Key(glfw::Key::Up, _, _, _) => {
        self.handle_direction_change(Direction::Up, event);
      }
      glfw::WindowEvent::Key(glfw::Key::Down, _, _, _) => {
        self.handle_direction_change(Direction::Down, event);
      }
      _ => {
        self.direction = Direction::Stationary;
      }
    }

    self.move_character(self.direction, 0.04);
  }

  pub fn move_character(&mut self, direction: Direction, amount: f32) {
    match direction {
      Direction::Right => {
        self.move_right(amount);
      }
      Direction::Left => {
        self.move_left(amount);
      }
      Direction::Up => {
        self.move_up(amount);
      }
      Direction::Down => {
        self.move_down(amount);
      }
      Direction::UpRight => {
        self.move_right(amount);
        self.move_up(amount);
      }
      Direction::UpLeft => {
        self.move_left(amount);
        self.move_up(amount);
      }
      Direction::DownRight => {
        self.move_right(amount);
        self.move_down(amount);
      }
      Direction::DownLeft => {
        self.move_left(amount);
        self.move_down(amount);
      }
      _ => {}
    }
  }

  pub fn flip_vertical(&mut self) {
    self.shape.flip_texture_corners_y()
  }

  pub fn flip_horizontal(&mut self) {
    self.shape.flip_texture_corners_x()
  }

  pub fn transform(&mut self, transformation_matrix: Matrix<f32>) {
    assert!(
      transformation_matrix.get_num_rows() == 4
        && transformation_matrix.get_num_columns() == 4,
      "Not a valid transformation_matrix"
    );

    self.transformation = transformation_matrix;
  }

  pub fn load_texture(&mut self) {
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

  pub fn get_height(&self) -> f32 {
    self.shape.get_height()
  }

  pub fn get_width(&self) -> f32 {
    self.shape.get_width()
  }

  pub fn move_up(&mut self, amount: f32) -> bool {
    let new_amount = self.shape.get_y() + amount;

    if new_amount >= 0.8 {
      return false;
    }

    self.shape.set_y(new_amount);

    return true;
  }

  pub fn move_down(&mut self, amount: f32) -> bool {
    let new_amount = self.shape.get_y() - amount;

    if new_amount <= -0.7 {
      return false;
    }

    self.shape.set_y(new_amount);

    return true;
  }

  pub fn move_right(&mut self, amount: f32) -> bool {
    let new_amount = self.shape.get_x() + amount;

    if new_amount > 0.7 {
      return false;
    }

    self.shape.set_x(new_amount);

    return true;
  }

  pub fn set_color_overlay(&mut self, color: Color) {
    self.shape.set_color(color);
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
  fn set_texture_uniform(&'a self, program: &ShaderProgram) -> () {
    self.texture.set_uniform(program);
  }

  fn get_corner_count(&'a self) -> i32 {
    self.shape.get_coordinate_corners().len() as i32
  }

  fn get_elements(&self) -> Vec<i32> {
    return vec![0, 1, 2, 2, 3, 0];
  }

  fn load_texture(&'a self) -> () {
    self.texture.load_texture();
  }

  fn get_vertices(&self) -> Vec<f32> {
    let mut vertices = Vec::<f32>::new();

    let shape = &self.shape;

    let coordinate_corners = &self.shape.get_coordinate_corners();
    let texture_corners = &self.shape.get_texture_corners();

    for i in 0..4 {
      // X, Y
      let [x, y] = coordinate_corners[i];
      vertices.push(x);
      vertices.push(y);

      // Color
      vertices.push(shape.get_color().r);
      vertices.push(shape.get_color().g);
      vertices.push(shape.get_color().b);
      vertices.push(shape.get_color().a);

      // Texture Cords
      let [tx, ty] = texture_corners[i];
      vertices.push(tx);
      vertices.push(ty);

      vertices.push(self.texture.texture_id as f32);

      for entry in self.transformation.get_inner_ptr() {
        vertices.push(entry.to_owned());
      }
    }
    return vertices;
  }
}

impl<'a, TShape> From<&Sprite<'a, TShape>> for Sprite<'a, TShape>
where
  TShape: Shape + Clone,
{
  fn from(sprite_ref: &Sprite<'a, TShape>) -> Self {
    Sprite::new(sprite_ref.shape.to_owned(), sprite_ref.texture.to_owned())
  }
}
