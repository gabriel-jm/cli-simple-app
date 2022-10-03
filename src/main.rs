mod terminal;
mod sections;
mod file;
mod state;

use std::{path::Path, fs::File, io::Write};

use file::get_file;
use sections::{home, dashboard};
use state::{Database, Account};

fn main() {
  if !Path::new("./database.json").exists() {
    File::create("./database.json")
      .expect("Unable to create file")
      .write(String::from("{\"current_user\":null,\"users\":[]}").as_ref())
      .expect("Unable to create file")
    ;
      
    return home(None, 1);
  }

  let json = get_file("./database.json");

  let stored_data: Database = serde_json::from_str(&json)
    .expect("Unable to parse JSON")
  ;

  if let None = stored_data.current_user {
    return home(None, 1);
  }

  let user = stored_data.users.into_iter().clone().find(|u| {
    return u.id.eq(&stored_data.current_user.clone().unwrap())
  });

  if let None = user {
    return home(None, 1);
  }

  let user = user.unwrap();

  dashboard(Account {
    id: user.id.clone(),
    name: user.name.clone()
  });
}
