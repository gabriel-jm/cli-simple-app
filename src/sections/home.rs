use console::{Term, Key};

use crate::terminal::in_out::{clear, flush_output};
use super::{create_account, not_found, login};
use crate::state::Account;
use super::components::header;

pub fn home(account: Option<Account>, position: u8) {
  clear();
  welcome(&account, position);
  flush_output();

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
  println!("[{}] - Log in an existing account", verify_position(position, 1));
  println!("[{}] - Create an account", verify_position(position, 2));
  println!("[{}] - Exit application", verify_position(position, 3));
}

fn verify_position(position: u8, target: u8) -> String {
  if position == target {
    return String::from("x");
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
