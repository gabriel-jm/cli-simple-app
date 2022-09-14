use std::{fs::File, io::Read};

use colored::Colorize;
use serde_json::Value;

use crate::{state::Account, terminal::in_out::{read_input, pause, clear}, sections::home, file::append_to_file};

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
    append_to_file("./src/data.json", "[]");
    println!("\n{}\n", "** User not found **".yellow());
    pause();
    home(account)
  }

  let users_list = match serde_json::from_str::<Value>(&json_data) {
    Ok(data) => data,
    Err(_) => {
      println!("\nUnable to read file data");
      Value::Array(vec![])
    }
  };

  println!("Position one: {}", users_list[0]);
}
