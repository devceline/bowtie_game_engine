use crate::{sprites::sprite::Sprite, shapes::rectangle::Rectangle, god_object::entity::{Entity, Component}, general::color::Color};

pub struct PlayableCharacter<'s> {
  sprite: Sprite<'s, Rectangle>,
  components: Vec<*mut dyn Component<'s>>
}

impl<'s> PlayableCharacter<'s> {

  pub fn new(sprite: Sprite<'s, Rectangle>) -> PlayableCharacter<'s> {
    PlayableCharacter { sprite, components: vec![] }
  }

  pub fn set_color_overlay(&mut self, color: Color) {
    self.sprite.set_color_overlay(color);
  }

  pub fn move_left(&mut self) { 
    self.sprite.move_left(0.02);
  }

  pub fn move_right(&mut self) { 
    self.sprite.move_right(0.02);
  }

  pub fn move_up(&mut self) { 
    self.sprite.move_up(0.02);
  }

  pub fn move_down(&mut self) { 
    self.sprite.move_down(0.02);
  }

  pub fn flip_horizontal(&mut self) {
    self.sprite.flip_horizontal();
  }

}

impl<'e> Entity<'e> for PlayableCharacter<'e> {
  fn get_drawable(&'e self) -> &'e dyn crate::sprites::drawable::Drawable<'e> {
    &self.sprite
  }

  fn get_x(&self) -> f32 {
      self.sprite.get_x()
  }

  fn get_y(&self) -> f32 {
      self.sprite.get_y()
  }

  fn get_width(&self) -> f32 {
      self.sprite.get_width()
  }

  fn get_height(&self) -> f32 {
      self.sprite.get_height()
  }

  fn load_components(&mut self, component: *mut dyn Component<'e>) {
    self.components.push(component);
  }

  fn get_components(&mut self) -> &Vec<*mut dyn crate::god_object::entity::Component<'e>> {
    &self.components
  }
}
