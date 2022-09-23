use crate::state::Account;

use super::components::header;

pub fn dashboard(account: Account) {
  header("Dashboard", &Some(account));

  println!("");
}
