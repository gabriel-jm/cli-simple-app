use crate::terminal::in_out::{clear, flush_output, read_input, print_multi_lines};
use super::{create_account, not_found};
use crate::state::Account;
use super::components::header;

pub fn home(account: Option<Account>) {
  clear();
  welcome(&account);
  flush_output();

  let command = read_input(None);

  match command.as_ref() {
    "c" => create_account(),
    "q" => println!("\nExiting..."),
    _ => not_found(account)
  }
}

fn welcome(account: &Option<Account>) {
  header("Home", account);
  println!("\nWelcome to the store");

  print_multi_lines(vec![
    vec!["\nCommands:"],
    vec![" c - Create an account"],
    vec![" l - Log in an existing account"],
    vec![" q - Exit application"]
  ]);

  print!("\n> ");
}
