mod rendering;
mod general;
mod bowtie;
mod gl_utils;
mod shapes;
mod sprites;
mod components;

pub mod math;

pub use bowtie::{controller::BowTie, entity::{Entity, Component, Message}};

pub use general::{direction::Direction, color::COLORS, color::Color};

pub use gl_utils::{gl_texture::{Texture, TextureOptions}, gl_translation::{TextureFilter, TextureWrap}};

pub use sprites::{sprite::Sprite, drawable::Drawable};

pub use shapes::{shape::Shape, rectangle::Rectangle};

pub mod premade_components {
  pub use crate::components::{gravity::GravityComponent, collide::CollisionComponent};
}

