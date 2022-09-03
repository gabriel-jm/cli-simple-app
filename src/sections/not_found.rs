use colored::Colorize;

use crate::terminal::in_out::{sleep, pause};
use super::home;

pub fn not_found() {
  println!("{}", "\n** Command not found **\n".bright_yellow());
  sleep(500);
  pause();
  home();
}
