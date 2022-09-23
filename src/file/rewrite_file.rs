use std::{io::{Write, Read}, fs::File};

pub fn rewrite_file(path: &str, data: &str) {
  let mut from_file = String::new();

  File::options()
    .read(true)
    .open(path)
    .expect("Unable to create new user")
    .read_to_string(&mut from_file)
    .expect("Unable to create new user")
  ;

  File::options()
    .write(true)
    .append(false)
    .open(path)
    .expect("Unable to create new user")
    .write(data.as_ref())
    .expect("Unable to create new user")
  ;
}
