extern crate bowtie;

use bowtie::{
  Component, Entity,
  COLORS,
  Drawable,
  Message,
  Texture, TextureOptions,
  Rectangle,
  Sprite,
};

pub struct Floor<'s> {
  sprite: Sprite<'s, Rectangle>,
  components: Vec<*mut dyn Component<'s>>,
}

impl<'s> Floor<'s> {
  pub fn new() -> Floor<'s> {
    Floor {
      sprite: Sprite::new(
        Rectangle::new(-1.0, -0.5, 2.0, 0.5, COLORS::White.into()),
        Texture::new("floor", TextureOptions::default()),
      ),
      components: vec![],
    }
  }
}

impl<'s> Entity<'s> for Floor<'s> {
  fn get_x(&self) -> f32 {
    self.sprite.get_x()
  }

  fn get_y(&self) -> f32 {
    self.sprite.get_y()
  }

  fn set_x(&mut self, x: f32) -> bool {
    false
  }
  fn set_y(&mut self, y: f32) -> bool {
    false
  }

  fn get_height(&self) -> f32 {
    self.sprite.get_height()
  }

  fn get_drawable(&'s self) -> &'s dyn Drawable<'s> {
    &self.sprite
  }

  fn get_components(&mut self) -> &Vec<*mut dyn Component<'s>> {
    &self.components
  }

  fn load_components(&mut self, component: *mut dyn Component<'s>) {
    self.components.push(component);
  }

  fn get_width(&self) -> f32 {
    self.sprite.get_width()
  }

  fn recieve_message(&mut self, message: Message) {}
}
