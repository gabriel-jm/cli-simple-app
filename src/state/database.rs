use serde::{Serialize, Deserialize};

use super::CreatedAccount;

#[derive(Serialize, Deserialize)]
pub struct Database {
  pub current_user: Option<String>,
  pub users: Vec<CreatedAccount>
}
