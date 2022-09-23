use console::{Term, Key};
use colored::{self, Colorize};

use crate::terminal::in_out::clear;
use super::{create_account, not_found, login};
use crate::state::Account;
use super::components::header;

pub fn home(account: Option<Account>, position: u8) {
  clear();
  welcome(&account, position);

  let stdout = Term::buffered_stdout();

  if let Ok(character) = stdout.read_key() {
    match character {
      Key::ArrowUp => home(account, u8::max(position - 1, 1)),
      Key::ArrowDown => home(account, u8::min(position + 1, 3)),
      Key::Enter => navigate(account, position),
      Key::Char(c) => {
        if c == 'q' {
          return ();
        }

        home(account, position);
      },
      _ => home(account, position)
    }
  }
}

fn welcome(account: &Option<Account>, position: u8) {
  header("Home", account);

  println!("\nWelcome!!!\n");
  println!("{}\n", "Press 'q' to exit.".bright_black());
  options_list(position, vec![
    "Log in an existing account",
    "Create an account",
    "Exit application"
  ])
}

fn options_list(position: u8, options: Vec<&str>) {
  for opt in 0..options.len() {
    println!(
      "{}{}{} {}",
      "(".bright_black(),
      verify_position(position, u8::try_from(opt + 1).expect("Size excedeed")),
      ")".bright_black(),
      options[opt]
    );
  }
}

fn verify_position(position: u8, target: u8) -> String {
  if position == target {
    return String::from("✔").green().to_string();
  } else {
    return String::from(" ");
  }
}

fn navigate(account: Option<Account>, position: u8) {
  match position {
    1 => login(account),
    2 => create_account(account),
    3 => println!("\nExiting..."),
    _ => not_found(account)
  }  
}
