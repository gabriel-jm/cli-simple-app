use serde::{Serialize, Deserialize};

use super::{CreatedAccount, ToDoList};

#[derive(Serialize, Deserialize)]
pub struct Database {
  pub current_user: Option<String>,
  pub users: Vec<CreatedAccount>,
  pub lists: Vec<ToDoList>
}
