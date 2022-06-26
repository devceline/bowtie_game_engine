use crate::{sprites::sprite::Sprite, shapes::rectangle::Rectangle, god_object::entity::Entity};

pub struct PlayableCharacter<'s> {
  pub sprite: Sprite<'s, Rectangle>
}

impl<'s> PlayableCharacter<'s> {
  pub fn move_right(&mut self) { 
    self.sprite.move_right(0.02);
  }
}

impl<'e> Entity<'e> for PlayableCharacter<'e> {
  fn get_drawable(&'e self) -> &'e dyn crate::sprites::drawable::Drawable<'e> {
    &self.sprite
  }
}
