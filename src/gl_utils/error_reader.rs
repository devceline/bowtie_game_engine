
pub enum GlError {
  ProgramError(u32),
  ShaderError(u32),
}

pub fn get_error(error: GlError, max_length: u32) {
  match error {
    GlError::ProgramError(id) => {
      unsafe { 
      }
    },
    GlError::ShaderError(id) => {
    },
  }
}
