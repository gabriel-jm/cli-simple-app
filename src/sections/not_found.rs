use crate::terminal::in_out::{sleep, pause};
use crate::terminal::colors::bright_yellow;
use super::home;

pub fn not_found() {
  println!("{}", bright_yellow("\n** Command not found **\n"));
  sleep(500);
  pause();
  home();
}
