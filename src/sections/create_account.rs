use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::state::Account;
use crate::terminal::in_out::{clear, read_input, pause};
use crate::file::{append_to_file, get_file};
use super::components::header;
use super::home;

#[derive(Serialize, Deserialize)]
struct CreatedAccount {
  id: String,
  name: String,
  password: String
}

pub fn create_account() {
  clear();
  header("Create account", &None);

  let name = read_input(Some("\nName: "));

  if name.eq("q") {
    println!("\nCancelling and returning to home...\n");
    pause();
    home(None, 1);
    return;
  }

  let password = read_input(Some("Password: "));

  if password.eq("q") {
    println!("\nCancelling and returning to home...\n");
    pause();
    home(None, 1);
    return;
  }

  let json = get_file("./src/data.json");

  let stored_data: Vec<CreatedAccount> = serde_json::from_str(&json)
    .expect("Unable to parse JSON")
  ;

  let exists = stored_data.into_iter().any(
    |user| name.eq(&user.name)
  );

  if exists {
    println!("\n{}", "❗ Name already in use\n".yellow());
    pause();
    return create_account();
  }

  let data = Account {
    id: uuid::Uuid::new_v4().to_string(),
    name
  };

  let json_data = json!({
    "id": data.id,
    "name": data.name,
    "password": password
  });

  append_to_file("./src/data.json", json_data.to_string().as_ref());

  home(Some(data), 1);
}
