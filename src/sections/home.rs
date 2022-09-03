use crate::terminal::in_out::{clear, flush_output, read_input, print_multi_lines};
use super::{create_account, not_found};

pub fn home() {
  clear();
  welcome();
  flush_output();

  let command = read_input(None);

  match command.as_ref() {
    "c" => create_account(),
    "q" => println!("\nExiting..."),
    _ => not_found()
  }
}

fn welcome() {
  print_multi_lines(vec![
    vec!["# Home\n", "cyan"],
    vec!["Welcome to the store"],
    vec!["\n\tYou're not current loged", "black"],
    vec!["\nCommands:"],
    vec![" c - Create an account"],
    vec![" l - Log in an existing account"],
    vec![" q - Exit application"]
  ]);

  print!("\n> ");
}
