mod bowtie;
mod components;
mod general;
mod gl_utils;
mod rendering;
mod shapes;
mod sprites;
mod window;

pub mod math;

pub use bowtie::{
  component::StandardComponent,
  controller::BowTie,
  entity::{Component, Entity, Message, StandardEntity},
};

pub use general::{color::Color, color::COLORS, direction::Direction};

pub use gl_utils::{
  gl_error_reader::init_debug_callback,
  gl_texture::{LoadableTexture, Texture, TextureOptions},
  gl_translation::{TextureFilter, TextureWrap},
};

pub use sprites::{drawable::Drawable, sprite::Sprite};

pub use shapes::{rectangle::Rectangle, shape::Shape};

pub mod premade_components {
  pub use crate::components::{
    collide::CollisionComponent, event::EventComponent,
    gravity::GravityComponent, keyboard_move::KeyboardMoveComponent,
  };
}

pub use window::window::{WindowMode, WindowConfig};
