use colored::Colorize;
use console::{Term, Key};
use create_todo_item::create_todo_item;

use crate::{state::{Account, ToDoItem}, terminal::in_out::{clear, read_input, pause, flush_output}, sections::{dashboard, create_todo_item::{self, CreateToDoItemProps}}};

use super::components::header;

pub struct CreateToDoListProps {
  pub account: Account,
  pub title: Option<String>,
  pub items: Vec<ToDoItem>
}

pub fn create_todo_list(props: CreateToDoListProps) {
  clear();
  header("Create To Do list", &Some(props.account.clone()));

  println!("{}", "\nEnter 'q' in any field to cancel".bright_black());

  let title = match props.title {
    Some(ref title) => String::from(title),
    None => read_input(Some("\nTitle: "))
  };

  if title.eq("q") {
    returning_to_dashboard(props.account);
    return;
  }

  println!("\nList title: {}", title.bold());

  print!("\n{} {}", "a".bold(), "add item to list  |".bright_black());
  print!("  {} {}", "q".bold(), "return to dashboard\n\n".bright_black());

  flush_output();

  let stdout = Term::buffered_stdout();

  if let Ok(character) = stdout.read_key() {
    if let Key::Char(c) = character {
      if c == 'a' {
        let items = create_todo_item(CreateToDoItemProps {
          account: &Some(props.account.clone()),
          list_title: &title[..],
          items: &props.items
        });

        return create_todo_list(CreateToDoListProps {
          title: Some(title),
          items,
          ..props
        });
      }

      if c == 'q' {
        return dashboard(props.account);
      }

      return create_todo_list(props)
    }
      
    return create_todo_list(props)
  }
}

fn returning_to_dashboard(account: Account) {
  println!("\nCancelling and returning to dashboard...\n");
  pause();
  dashboard(account);
}
