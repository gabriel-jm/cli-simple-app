use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::{state::Account, terminal::in_out::{read_input, pause, clear}, sections::home, file::get_file};

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

  println!("{}", "\nEnter 'q' in any field to cancel".bright_black());

  let name = read_input(Some("\nName: "));

  if name.eq("q") {
    return return_to_home(account);
  }

  let password = read_input(Some("Password: "));

  if password.eq("q") {
    return return_to_home(account);
  }

  let json_data = get_file("./data.json");

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
    home(Some(Account { id: user_data.id, name: user_data.name }), 1)
  } else {
    on_user_not_found();
    home(account, 1);
  }
}

fn on_user_not_found() {
  println!("\n{}\n", "** User not found **".yellow());
  pause();
}

fn return_to_home(account: Option<Account>) {
  println!("\nCancelling and returning to home...\n");
  pause();
  home(account, 1);
}
