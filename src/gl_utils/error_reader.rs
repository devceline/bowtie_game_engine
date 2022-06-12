pub fn get_error() {
    unsafe {
      let err = gl::GetError();
    println!("{}", err);
    }
}
