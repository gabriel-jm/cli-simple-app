use serde_json::json;
use crate::state::Account;
use crate::terminal::in_out::{clear, read_input, pause};
use crate::file::append_to_file;
use super::components::header;
use super::home;

pub fn create_account() {
  clear();
  header("Create account", &None);

  let name = read_input(Some("\nEnter your name: "));

  if name.eq("q") {
    println!("\nCancelling and returning to home...\n");
    pause();
    home(None);
    return;
  }

  let password = read_input(Some("Enter your password: "));

  if password.eq("q") {
    println!("\nCancelling and returning to home...\n");
    pause();
    home(None);
    return;
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

  home(Some(data));
}
