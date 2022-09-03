use std::{io::{Write, Read}, fs::File, path::Path};

pub fn append_to_file(path: &str, data: &str) {
  if !Path::new(path).exists() {
    File::create(path)
      .expect("Unable to create file")
      .write(String::from("[]").as_ref())
      .expect("Unable to create file")
    ;
  }

  let mut from_file = String::new();

  File::options()
    .read(true)
    .open(path)
    .expect("Unable to create new user")
    .read_to_string(&mut from_file)
    .expect("Unable to create new user")
  ;

  let new_data = 
    if from_file.len() == 2 {
      "[".to_owned() + data.as_ref() + "]"
    } else {
      from_file.replace("]", ",") + data.as_ref() + "]"
    }
  ;

  File::options()
    .write(true)
    .open(path)
    .expect("Unable to create new user")
    .write(new_data.as_ref())
    .expect("Unable to create new user")
  ;
}
