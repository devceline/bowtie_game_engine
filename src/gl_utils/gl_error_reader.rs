extern crate gl;

pub enum GlErrorResult {
  NoError,
  Error(String),
}

pub enum GlError {
  ShaderError(u32),
}

pub fn pull_errors() {
  loop {
    let err = unsafe { gl::GetError() };
    if err != gl::NO_ERROR {
      println!("{}", err);
    } else {
      break;
    }
  }
}

fn buffer_to_string(buffer: &Vec<i8>) -> String {
  return String::from_utf8(
    buffer
      .iter()
      .map(|s| s.to_owned() as u8)
      .collect::<Vec<u8>>(),
  )
  .expect("Could not transform buffer to string");
}

pub fn get_error(error: GlError, max_length: i32) -> GlErrorResult {
  let mut buffer: Vec<i8> = Vec::new();
  buffer.reserve(max_length as usize);
  let mut buffer_len = 0;
  for _ in 0..max_length {
    buffer.push(0);
  }

  match error {
    GlError::ShaderError(shader_id) => unsafe {
      let mut status = -1;
      gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut status);

      if (status as u8) != gl::TRUE {
        gl::GetShaderInfoLog(
          shader_id,
          max_length,
          &mut buffer_len,
          &mut buffer[0],
        );
        return GlErrorResult::Error(buffer_to_string(&buffer));
      } else {
        return GlErrorResult::NoError;
      }
    },
  }
}

extern "system" fn debug_message_callback(
  source: u32,
  message_type: u32,
  _id: u32,
  severity: u32,
  length: i32,
  message: *const gl::types::GLchar,
  _user_param: *mut gl::types::GLvoid,
) {
  unsafe {
    let mut buffer: Vec<i8> = Vec::new();
    buffer.reserve(length as usize);

    for i in 0..length {
      buffer.push(std::ptr::read_volatile(message.offset(i as isize)));
    }

    println!(
      "(Severity {}) (Type {}): (Source {}) {:?}",
      severity,
      message_type,
      source,
      buffer_to_string(&buffer)
    );
  }
}

pub fn init_debug_callback() {
  unsafe {
    gl::Enable(gl::DEBUG_OUTPUT);
    gl::DebugMessageCallback(
      Some(debug_message_callback),
      0 as *const gl::types::GLvoid,
    );
  }
}
