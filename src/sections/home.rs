use console::{Term, Key};
use colored::{self, Colorize};

use super::{create_account, not_found, login};
use crate::{state::Account, terminal::in_out::clear};
use super::components::header;

pub fn home(account: Option<Account>, position: u8) {
  clear();
  header("Home", &account);

  println!("\nWelcome!!!\n");
  println!("{}\n", "Press 'q' to exit.".bright_black());
  options_list(position, vec![
    "Log in an existing account",
    "Create an account",
    "Exit application"
  ]);

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

fn options_list(position: u8, options: Vec<&str>) {
  for opt in 0..options.len() {
    if position == u8::try_from(opt + 1).expect("Size exceeded") {
      println!("{} {}", "| ".bright_green(), options[opt].bright_green());
      continue;
    }

    println!("   {}", options[opt]);
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
