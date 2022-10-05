use crate::{state::Account, terminal::in_out::clear};

use super::components::header;

pub fn create_todo_list(account: Account) {
  clear();
  header("Create To Do list", &Some(account.clone()));

  println!("");
}
