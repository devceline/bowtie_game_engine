extern crate gl;
extern crate png;

use std::fs::File;

use super::shader_creator::{ShaderProgram, Uniform};

use super::gl_translation::{DataType, TextureFilter, TextureWrap, ToGl};

static mut TEXTURE_COUNT: u32 = 0;

pub struct Texture {
  _id: u32,
}

pub struct TextureOptions {
  wrap: TextureWrap,
  filter: TextureFilter,
}

impl TextureOptions {
  pub fn defaults() -> TextureOptions {
    TextureOptions {
      wrap: TextureWrap::ClampToEdge,
      filter: TextureFilter::LinearMipmap,
    }
  }
}

impl Texture {
  pub fn new() -> Texture {
    unsafe {
      TEXTURE_COUNT = TEXTURE_COUNT + 1;
    }

    let mut id: u32 = 0;

    unsafe { gl::GenTextures(1, &mut id) };

    Texture { _id: id }
  }

  fn get_image_location(location: &str) -> String {
    let mut base_url = String::from("./images/");
    base_url.push_str(location);
    base_url.push_str(".png");
    return base_url;
  }

  pub fn load_texture(&self, image_name: &str, options: TextureOptions, program: &ShaderProgram) {
    unsafe {
      gl::ActiveTexture(gl::TEXTURE0 + TEXTURE_COUNT);
      gl::BindTexture(gl::TEXTURE_2D, self._id);

      // Loading file bytes
      let decoder = png::Decoder::new(File::open(Texture::get_image_location(image_name)).unwrap());

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

      program.set_uniform(Uniform {
        name: String::from(image_name),
        data_type: DataType::Int,
        count: 1,
        values: vec![TEXTURE_COUNT],
      });

      gl::GenerateMipmap(gl::TEXTURE_2D);

      // Wrap
      gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_WRAP_S,
        options.wrap.to_gl() as i32,
      );
      gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_WRAP_T,
        options.wrap.to_gl() as i32,
      );

      // Filter
      gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MAG_FILTER,
        options.filter.to_gl() as i32,
      );
      gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MIN_FILTER,
        options.filter.to_gl() as i32,
      );
    };
  }
}
