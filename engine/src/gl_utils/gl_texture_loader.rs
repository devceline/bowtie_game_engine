use crate::{LoadableTexture, Texture};
use std::collections::HashMap;

use super::shader_creator::ShaderProgram;

#[derive(Clone)]
pub struct TextureLoader {
  loaded_textures: HashMap<u32, bool>,
}

impl TextureLoader {
  pub fn new() -> TextureLoader {
    TextureLoader {
      loaded_textures: HashMap::new(),
    }
  }
  pub fn load_texture(&mut self, texture: Texture, program: &ShaderProgram) {
    if texture.texture_id < 0 {
      return;
    }

    let entry = self
      .loaded_textures
      .entry(texture.texture_id as u32)
      .or_insert(false);

    if !*entry {
      texture.load_texture(program);
      *entry = true;
    }
  }
  pub fn load_textures(
    &mut self,
    textures: Vec<Texture>,
    program: &ShaderProgram,
  ) {
    for texture in textures {
      self.load_texture(texture.to_owned(), &program);
    }
  }
}

impl Drop for TextureLoader {
  fn drop(&mut self) {
    println!("Dropping loader");
    for texture in self.loaded_textures.keys() {
      //unsafe { gl::DeleteTextures(1, texture) };
    }
  }
}
