use std::{fs::File, io::Read};

use colored::Colorize;

use crate::{state::Account, terminal::in_out::{read_input, pause, clear}, sections::home};

use super::components::header;

pub fn login(account: Option<Account>) {
  clear();
  header("Login", &account);

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
    println!("\n{}\n", "** User not found **".yellow());
    pause();
    home(account);
    return;
  }
}
