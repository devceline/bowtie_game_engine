extern crate gl;

pub enum GlErrorResult {
    NoError,
    Error(String),
}

pub enum GlError {
    ShaderError(u32),
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
                gl::GetShaderInfoLog(shader_id, max_length, &mut buffer_len, &mut buffer[0]);
                return GlErrorResult::Error(buffer_to_string(&buffer));
            } else {
                return GlErrorResult::NoError;
            }
        },
    }
}
