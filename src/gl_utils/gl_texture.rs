extern crate gl;
extern crate png;

use std::fs::File;

use super::shader_creator::{ShaderProgram, Uniform};

use super::gl_translation::{DataType, TextureFilter, TextureWrap, ToGl};

static mut TEXTURE_COUNT: u32 = 0;

#[derive(Debug)]
pub struct TextureOptions {
  wrap: TextureWrap,
  min_filter: TextureFilter,
  mag_filter: TextureFilter,
}

#[derive(Debug)]
pub struct Texture {
  id: i32,
  pub texture_id: i32,
  options: TextureOptions,
  pub image_name: String,
}

impl TextureOptions {
  pub fn defaults() -> TextureOptions {
    TextureOptions {
      wrap: TextureWrap::ClampToEdge,
      min_filter: TextureFilter::LinearMipmap,
      mag_filter: TextureFilter::Linear,
    }
  }
}

impl Texture {
  pub fn new(
    image_name: &str,
    options: TextureOptions,
    _program: &ShaderProgram,
  ) -> Texture {
    unsafe {
      let mut id: u32 = 0;
      gl::GenTextures(1, &mut id);

      println!("Created new texture, with name and id: {}, {}", image_name, TEXTURE_COUNT );
      let tex = Texture {
        texture_id: TEXTURE_COUNT as i32,
        id: id as i32,
        options,
        image_name: String::from(image_name),
      };
      TEXTURE_COUNT = TEXTURE_COUNT + 1;


      return tex;
    }
  }

  pub fn none() -> Texture {
    Texture {
      texture_id: -1,
      id: -1,
      options: TextureOptions::defaults(),
      image_name: String::from(""),
    }
  }

  fn get_image_location(location: &str) -> String {
    let mut base_url = String::from("./images/");
    base_url.push_str(location);
    base_url.push_str(".png");
    return base_url;
  }

  pub fn set_uniform(&self, program: &ShaderProgram) {
      program.set_uniform(Uniform {
        name: format!("tex{}_sampler", self.texture_id),
        data_type: DataType::Int,
        count: 1,
        values: vec![self.texture_id],
      });
  }

  pub fn load_texture(&self) {
    unsafe {
      gl::ActiveTexture(gl::TEXTURE0 + (self.texture_id as u32));
      gl::BindTexture(gl::TEXTURE_2D, self.id as u32);

      println!("Bound to texture {}", self.texture_id);

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
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    let id_u32 = self.id as u32;
    unsafe { gl::DeleteTextures(1, &id_u32) };
  }
}
