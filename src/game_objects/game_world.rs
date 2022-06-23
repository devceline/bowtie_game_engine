use crate::god_object::entity::Entity;
use crate::shapes::rectangle::Rectangle;
use crate::sprites::drawable::Drawable;
use crate::sprites::sprite::Sprite;

#[derive(Debug)]
pub struct GameWorld<'a> {
  floor: Sprite<'a, Rectangle>,
  sky: Sprite<'a, Rectangle>,
}


impl<'a> GameWorld<'a> {
  pub fn new(
    floor: Sprite<'a, Rectangle>,
    sky: Sprite<'a, Rectangle>,
  ) -> GameWorld<'a> {
    GameWorld {
      floor: floor.to_owned(),
      sky: sky.to_owned(),
    }
  }
}

impl<'a> Drawable<'a> for GameWorld<'a> {
  fn set_texture_uniform(
    &'a self,
    program: &crate::gl_utils::shader_creator::ShaderProgram,
  ) -> () {
    self.sky.set_texture_uniform(program);
    self.floor.set_texture_uniform(program);
  }

  fn get_corner_count(&'a self) -> i32 {
    (self.sky.get_corner_count() + self.floor.get_corner_count()) as i32
  }

  fn get_vertices(&self) -> Vec<f32> {
    let mut sky_vertices = self.sky.get_vertices();
    let mut floor_vertices = self.floor.get_vertices();
    sky_vertices.append(&mut floor_vertices);
    sky_vertices
  }

  fn get_elements(&self) -> Vec<i32> {
    let mut sky_elements = self.sky.get_elements();
    let mut floor_elements: Vec<i32> = self
      .floor
      .get_elements()
      .iter()
      .map(|elem| elem + self.sky.get_corner_count())
      .collect();
    sky_elements.append(&mut floor_elements);
    sky_elements
  }

  fn load_texture(&'a self) -> () {
    self.sky.load_texture();
    self.floor.load_texture();
  }
}

impl<'a> Entity<'a> for GameWorld<'a> {
  fn get_drawable(&'a self) -> *const dyn Drawable<'a> {
    let floo: *const dyn Drawable<'a> = &self.floor;
    return floo;
  }
}
