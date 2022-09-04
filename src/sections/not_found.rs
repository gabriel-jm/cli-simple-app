use colored::Colorize;

use crate::{terminal::in_out::{sleep, pause}, state::Account};
use super::home;

pub fn not_found(account: Option<Account>) {
  println!("{}", "\n** Command not found **\n".bright_yellow());
  sleep(500);
  pause();
  home(account);
}
