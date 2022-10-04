use colored::Colorize;
use serde_json::json;

use crate::{state::{Account, Database}, terminal::in_out::{read_input, pause, clear}, sections::home, file::{get_file, rewrite_file}};

use super::components::header;

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

  let json_data = get_file("./database.json");

  if json_data.is_empty() {
    return on_user_not_found();
  }

  let mut database: Database = serde_json::from_str(&json_data)
    .expect("JSON parse error")
  ;

  let user = database.users.clone().into_iter().find(
    |user| name.eq(&user.name) && password.eq(&user.password)
  );

  if let Some(user_data) = user {
    database.current_user = Some(user_data.id.clone());
    rewrite_file("./database.json", json!(database).to_string().as_ref());

    return home(Some(Account {
      id: user_data.id,
      name: user_data.name
    }), 1);
  }
  
  on_user_not_found();
  home(account, 1);
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
