mod terminal;
mod sections;
mod file;
mod state;

use std::{path::Path, fs::File, io::Write};

use sections::home;

fn main() {
  if !Path::new("./database.json").exists() {
    File::create("./database.json")
      .expect("Unable to create file")
      .write(String::from("{\"current_user\":null,\"users\":[]}").as_ref())
      .expect("Unable to create file")
    ;
  }

  home(None, 1);
}
