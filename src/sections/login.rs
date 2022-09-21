use std::{fs::File, io::Read};

use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::{state::Account, terminal::in_out::{read_input, pause, clear}, sections::home, file::append_to_file};

use super::components::header;

#[derive(Serialize, Deserialize)]
struct CreatedAccount {
  id: String,
  name: String,
  password: String
}

pub fn login(account: Option<Account>) {
  clear();
  header("Login", &account);

  let name = read_input(Some("\nName: "));

  if name.eq("q") {
    println!("\nCancelling and returning to home...\n");
    pause();
    home(None);
    return;
  }

  let password = read_input(Some("Password: "));

  if password.eq("q") {
    println!("\nCancelling and returning to home...\n");
    pause();
    home(None);
    return;
  }

  let mut json_data = String::new();

  File::options()
    .create(true)
    .append(true)
    .read(true)
    .open("./src/data.json")
    .expect("Unable to open file")
    .read_to_string(&mut json_data)
    .expect("Unable to read file")
  ;

  if json_data.is_empty() {
    return on_user_not_found();
  }

  let users_list: Vec<CreatedAccount> = serde_json::from_str(&json_data)
    .expect("JSON parse error")
  ;

  let user = users_list.into_iter().find(
    |user| name.eq(&user.name) && password.eq(&user.password)
  );

  if let Some(user_data) = user {
    home(Some(Account { id: user_data.id, name: user_data.name }))
  } else {
    on_user_not_found()
  }
}

fn on_user_not_found() {
  append_to_file("./src/data.json", "[]");
  println!("\n{}\n", "** User not found **".yellow());
  pause();
  home(None);
}
