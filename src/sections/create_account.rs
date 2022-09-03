use crate::terminal::in_out::{clear, read_input, pause};
use crate::terminal::colors::{bright_cyan};
use crate::file::append_to_file;
use super::home;

pub fn create_account() {
  clear();
  println!("{}", bright_cyan("# Create account\n\n"));

  let name = read_input(Some("Enter your name: "));
  let password = read_input(Some("Enter your password: "));

  if name.eq("q") || password.eq("q") {
    println!("\nCancelling and returning to home...\n");
    pause();
    home();
    return;
  }

  let data = "{".to_owned()
    + "\"id\":\""
    + uuid::Uuid::new_v4().to_string().as_ref()
    + "\",\"name\":\""
    + name.as_ref()
    + "\",\"password\":\""
    + password.as_ref()
    + "\"}"
  ;

  append_to_file("./src/data.json", data.as_ref());
  home();
}
