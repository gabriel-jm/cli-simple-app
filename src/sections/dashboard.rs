use console::{Term, Key};

use crate::{state::{Account, Database}, sections::home, file::get_file, terminal::in_out::clear};

use super::components::header;
use colored::Colorize;

pub fn dashboard(account: Account) {
  clear();
  header("Dashboard", &Some(account.clone()));

  let stored_data: Database = serde_json::from_str(&get_file("./database.json"))
    .expect("Unable to get file data")
  ;

  println!("\nYour have a total of {} to do lists.", stored_data.lists.len());

  println!("{}", "\nPress 'o' to logout".bright_black());
  println!("{}", "Press 'n' to create a new to do list".bright_black());

  let stdout = Term::buffered_stdout();

  if let Ok(character) = stdout.read_key() {
    match character {
      Key::Char(c) => {
        if c == 'o' {
          return home(None, 1);
        }

        if c == 'q' {
          return ();
        }

        dashboard(account)
      },
      _ => dashboard(account)
    }
  }
}
