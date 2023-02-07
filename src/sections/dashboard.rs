use console::{Term, Key};

use crate::{state::{Account, Database}, sections::{home, create_todo_list::{create_todo_list, CreateToDoListProps}}, file::get_file, terminal::in_out::{clear, flush_output}};

use super::components::header;
use colored::Colorize;

pub fn dashboard(account: Account) {
  clear();
  header("Dashboard", &Some(account.clone()));

  let stored_data: Database = serde_json::from_str(&get_file("./database.json"))
    .expect("Unable to get file data")
  ;

  println!("\nYour have a total of {} To Do lists.", stored_data.lists.len());

  print!("\n{} {}", "n".bold(), "to create a new to do list  |".bright_black());
  print!("  {} {}", "o".bold(), "to logout  |".bright_black());
  print!("  {} {}", "q".bold(), "exit\n\n".bright_black());

  flush_output();

  let stdout = Term::buffered_stdout();

  if let Ok(character) = stdout.read_key() {
    match character {
      Key::Char(c) => {
        if c == 'n' {
          return create_todo_list(CreateToDoListProps {
            account,
            title: None,
            items: None
          });
        }
        
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
