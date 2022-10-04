use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct Account {
  pub id: String,
  pub name: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreatedAccount {
  pub id: String,
  pub name: String,
  pub password: String
}
