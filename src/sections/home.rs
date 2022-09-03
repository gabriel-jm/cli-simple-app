use crate::terminal::in_out::{clear, flush_output, read_input, print_multi_lines};
use crate::terminal::colors::bright_cyan;
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
    bright_cyan("# Home\n").as_ref(),
    "Welcome to the store",
    "\n\tYou're not current loged",
    "\nCommands:",
    " c - Create an account",
    " l - Log in an existing account",
    " q - Exit application"
  ]);

  print!("\n> ");
}
