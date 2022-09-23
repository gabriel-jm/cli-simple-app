mod terminal;
mod sections;
mod file;
mod state;

use std::{path::Path, fs::File, io::Write};

use sections::home;

fn main() {
  if !Path::new("./data.json").exists() {
    File::create("./data.json")
      .expect("Unable to create file")
      .write(String::from("[]").as_ref())
      .expect("Unable to create file")
    ;
  }

  home(None, 1);
}
