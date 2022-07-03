use std::env;
use std::fs;

fn main() {
  let out_dir = env::var_os("OUT_DIR").unwrap().into_string().unwrap();
  let out_shaders = format!("{}/shaders/", out_dir);
  match fs::create_dir(out_shaders.as_str()) {
    _ => {}
  };

  for shader_dir in fs::read_dir("./shaders/").unwrap().into_iter() {
    let entry = shader_dir.unwrap();
    let entry_name = entry.file_name().into_string().unwrap();
    match fs::create_dir(format!("{}{}", out_shaders, entry_name).as_str()) {
      _ => {}
    };
    println!("Reading ./shaders/{}", entry_name);
    for file in fs::read_dir(format!("./shaders/{}", entry_name))
      .unwrap()
      .into_iter()
    {
      let file_name = file.unwrap().file_name().into_string().unwrap();
      fs::copy(
        format!("./shaders/{}/{}", entry_name, file_name),
        format!("{}{}/{}", out_shaders, entry_name, file_name),
      )
      .expect("");
    }
  }
}
