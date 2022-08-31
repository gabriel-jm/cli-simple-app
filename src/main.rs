use std::{io::{self, Write}, process};

fn main() {
  clear();
  welcome();
  io::stdout().flush().expect("Unexpected error on read line");

  let command = read_input();

  println!("Input: [{}]", command);

  if command.eq("signin") {
    clear();
    println!("Sign in!!!");
  }
}

fn welcome() {
  print_multi_lines([
    "Welcome to the store",
    "\tYou'r not current loged",
    "\nCommands:",
    "signin - Create an account",
    "login - Log in an existing account"
  ].to_vec());

  print!("> ");
}

fn print_multi_lines(texts: Vec<&str>) {
  for text in texts {
    println!("{}", text);
  }
}

fn clear() {
  process::Command::new("clear").status().unwrap();
}

fn read_input() -> String {
  let mut data = String::new();

  io::stdin()
    .read_line(&mut data)
    .expect("Failed to read line.")
  ;

  return String::from(data.trim());
}
