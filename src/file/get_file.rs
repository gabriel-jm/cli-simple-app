use std::{fs::File, io::Read};
pub fn get_file(path: &str) -> String {
  let mut json_data = String::new();

  File::options()
    .create(true)
    .append(true)
    .read(true)
    .open(path)
    .expect("Unable to open file")
    .read_to_string(&mut json_data)
    .expect("Unable to read file")
  ;

  return json_data.to_owned();
}
