extern crate gl;
extern crate png;

use std::fs::File;

use super::shader_creator::ShaderProgram;

use super::uniform::{SettableUniform, UniformInteger};

use super::gl_translation::{TextureFilter, TextureWrap, ToGl};

static mut TEXTURE_COUNT: u32 = 0;

pub trait LoadableTexture {
  fn load_texture(&self);
}

#[derive(Debug, Copy, Clone)]
pub struct TextureOptions {
  pub wrap: TextureWrap,
  pub min_filter: TextureFilter,
  pub mag_filter: TextureFilter,
}

#[derive(Debug, Clone)]
/// OpenGL texture. Upon creation, it defines a sampler uniform and initializes
/// a texture buffer, loading it with wit the file's data.
/// Current supported format(s): png
pub struct Texture {
  pub texture_id: i32,
  pub image_name: String,
  id: i32,
  options: TextureOptions,
  is_from_ref: bool,
}

impl Default for TextureOptions {
  /**
   * Creates linear filtered texture, clamped to edge
   */
  fn default() -> Self {
    TextureOptions {
      wrap: TextureWrap::ClampToEdge,
      min_filter: TextureFilter::LinearMipmap,
      mag_filter: TextureFilter::Linear,
    }
  }
}

impl From<&Texture> for Texture {
  /*
   * Loads texture from texutre reference
   * NOTE: Texture has to be loaded before hand
   */
  fn from(texture_ref: &Texture) -> Self {
    Texture {
      texture_id: texture_ref.texture_id,
      id: texture_ref.id,
      options: texture_ref.options,
      image_name: String::from(texture_ref.image_name.as_str()),
      is_from_ref: true,
    }
  }
}

impl Texture {
  /// Creates a new texture ready to be loaded
  /// It will generate a texture buffer on the gl state machine
  pub fn new(image_name: &str, options: TextureOptions) -> Texture {
    unsafe {
      let mut id: u32 = 0;
      gl::GenTextures(1, &mut id);

      // Create a texture that is ready to be loaded
      let tex = Texture {
        texture_id: TEXTURE_COUNT as i32,
        id: id as i32,
        options,
        image_name: String::from(image_name),
        is_from_ref: false,
      };

      // Incrementing texture count to have accurate Texture ID
      TEXTURE_COUNT = TEXTURE_COUNT + 1;

      return tex;
    }
  }

  /// Function used to denote that lack of texture
  pub fn none() -> Texture {
    Texture {
      texture_id: -1,
      id: -1,
      options: TextureOptions::default(),
      image_name: String::from(""),
      is_from_ref: true,
    }
  }

  fn get_image_location(image_name: &str) -> String {
    return format!("./images/{image_name}.png");
  }

  /// Sets the sampler fragment shader Uniform
  pub fn set_uniform(&self, program: &ShaderProgram) {
    if self.texture_id == -1 {
      return;
    }
    let uniform = UniformInteger::new(
      format!("tex{}_sampler", self.texture_id).as_str(),
      self.texture_id,
    );
    program.set_uniform(&uniform);
  }
}

impl LoadableTexture for Texture {
  fn load_texture(&self) {
    if self.is_from_ref {
      return;
    }
    unsafe {
      // BindTexture requires a specific texture to be activated first
      gl::ActiveTexture(gl::TEXTURE0 + (self.texture_id as u32));
      gl::BindTexture(gl::TEXTURE_2D, self.id as u32);

      // Loading file bytes
      let decoder = png::Decoder::new(
        File::open(Texture::get_image_location(&self.image_name)).unwrap(),
      );

      let (info, mut reader) = decoder.read_info().unwrap();
      let mut buf = vec![0; info.buffer_size()];
      reader.next_frame(&mut buf).unwrap();

      // Loading image into gl
      gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        info.color_type.to_gl() as i32,
        info.width as i32,
        info.height as i32,
        0,
        info.color_type.to_gl(),
        gl::UNSIGNED_BYTE,
        buf.as_ptr() as *const gl::types::GLvoid,
      );

      // Using mipmaps for performance
      gl::GenerateMipmap(gl::TEXTURE_2D);

      // Wrap
      gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_WRAP_S,
        self.options.wrap.to_gl() as i32,
      );
      gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_WRAP_T,
        self.options.wrap.to_gl() as i32,
      );

      // Filter
      gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MAG_FILTER,
        self.options.mag_filter.to_gl() as i32,
      );
      gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MIN_FILTER,
        self.options.min_filter.to_gl() as i32,
      );
    };
    // self.is_loaded = true;
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    let id_u32 = self.id as u32;
    unsafe { gl::DeleteTextures(1, &id_u32) };
  }
}
