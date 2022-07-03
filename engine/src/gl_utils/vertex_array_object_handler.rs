extern crate gl;

pub struct VertexArrayObject {
  vao_id: u32,
}

impl VertexArrayObject {
  pub fn new() -> VertexArrayObject {
    let mut vao: u32 = 0;
    unsafe {
      gl::GenVertexArrays(1, &mut vao);
      gl::BindVertexArray(vao);
    }
    return VertexArrayObject { vao_id: vao };
  }
}

impl Drop for VertexArrayObject {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteVertexArrays(1, &self.vao_id);
    }
  }
}
