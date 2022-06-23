use std::env;
use std::fs;

fn main() {
  let out_dir = env::var_os("OUT_DIR").unwrap().into_string().unwrap();
  let out_images = format!("{}/../../../images/", out_dir);
  let out_shaders = format!("{}/../../../shaders/", out_dir);
  match fs::create_dir(out_images.as_str()) {
    _ => {}
  };
  match fs::create_dir(out_shaders.as_str()) {
    _ => {}
  };
  for file in fs::read_dir("./images/").unwrap().into_iter() {
    let entry = file.unwrap();
    let entry_name = entry.file_name().into_string().unwrap();
    fs::copy(
      format!("./images/{}", entry_name),
      format!("{}/{}", out_images, entry_name),
    )
    .unwrap();
  }

  for shader_dir in fs::read_dir("./shaders/").unwrap().into_iter() {
    let entry = shader_dir.unwrap();
    let entry_name = entry.file_name().into_string().unwrap();
    match fs::create_dir(format!("{}{}", out_shaders, entry_name).as_str()) {
      _ => {}
    };
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
