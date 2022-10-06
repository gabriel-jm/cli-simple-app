use colored::Colorize;
use crate::sections::dashboard;
use crate::state::{Account, CreatedAccount, Database};
use crate::terminal::in_out::{clear, read_input, pause};
use crate::file::{rewrite_file, get_file};
use super::components::header;
use super::home;

pub fn create_account(account: Option<Account>) {
  clear();
  header("Create account", &account);

  println!("{}", "\nEnter 'q' in any field to cancel".bright_black());

  let name = read_input(Some("\nName: "));

  if name.eq("q") {
    returning_to_home();
    return;
  }

  let password = read_input(Some("Password: "));

  if password.eq("q") {
    returning_to_home();
    return;
  }

  let json = get_file("./database.json");

  let mut stored_data: Database = serde_json::from_str(&json)
    .expect("Unable to parse JSON")
  ;

  let exists = stored_data.users.clone().into_iter().any(
    |user| name.eq(&user.name)
  );

  if exists {
    println!("\n{}", "❗ Name already in use\n".yellow());
    pause();
    return create_account(account);
  }

  let data = Account {
    id: uuid::Uuid::new_v4().to_string(),
    name
  };

  stored_data.users.push(CreatedAccount {
    id: data.id.clone(),
    name: data.name.clone(),
    password
  });

  rewrite_file(
    "./database.json",
    serde_json::json!(&stored_data).to_string().as_ref()
  );

  dashboard(data);
}

fn returning_to_home() {
  println!("\nCancelling and returning to home...\n");
  pause();
  home(None, 1);
}
