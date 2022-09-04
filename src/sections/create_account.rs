use crate::state::Account;
use crate::terminal::in_out::{clear, read_input, pause};
use crate::file::append_to_file;
use super::components::header;
use super::home;

pub fn create_account() {
  clear();
  header("Create account", &None);

  let name = read_input(Some("\nEnter your name: "));
  let password = read_input(Some("Enter your password: "));

  if name.eq("q") || password.eq("q") {
    println!("\nCancelling and returning to home...\n");
    pause();
    home(None);
    return;
  }

  let data = Account {
    id: uuid::Uuid::new_v4().to_string(),
    name
  };

  let json_data = format!(
    "{{\"id\":\"{}\",\"name\":\"{}\",\"password\":\"{}\"}}",
    data.id,
    data.name,
    password,
  );

  append_to_file("./src/data.json", json_data.as_ref());
  home(Some(data));
}
