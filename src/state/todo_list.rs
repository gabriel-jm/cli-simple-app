use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ToDoList {
  pub title: String,
  pub user_id: String,
  pub items: Vec<ToDoItem>
}

#[derive(Serialize, Deserialize)]
pub struct ToDoItem {
  pub name: String,
  pub checked: bool
}
