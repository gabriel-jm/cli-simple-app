use std::{io::{self, Write, Read}, process, thread, time};

fn main() {
  home();
}

fn home() {
  clear();
  welcome();
  flush_output();

  let command = read_input(None);

  match command.as_ref() {
    "signin" => sign_in(),
    "q" => {
      println!("\nExiting...")
    },
    _ => {
      println!("\x1b[93m\n** Command not found **\n\x1b[0m");
      thread::sleep(time::Duration::from_millis(500));
      pause();
      home();
    }
  }
}

fn sign_in() {
  clear();
  println!("# Sign In\n\n");
  let name = read_input(Some("Enter your name: "));
  let password = read_input(Some("Enter your password: "));

  println!("Name: {}, Password: {}", name, password);

  home();
}

fn welcome() {
  print_multi_lines([
    "# Home\n",
    "Welcome to the store",
    "\n\tYou're not current loged",
    "\nCommands:",
    " signin - Create an account",
    " login - Log in an existing account",
    " q - Exit application"
  ].to_vec());

  print!("\n> ");
}

fn pause() {
  let mut stdin = io::stdin();
  let mut stdout = io::stdout();

  write!(stdout, "Press ENTER to continue...").unwrap();
  stdout.flush().unwrap();

  let _ = stdin.read(&mut [0u8]).unwrap();
}

fn print_multi_lines(texts: Vec<&str>) {
  for text in texts {
    println!("{}", text);
  }
}

fn clear() {
  process::Command::new("clear").status().unwrap();
}

fn flush_output() {
  io::stdout().flush().expect("Unexpected error on read line");
}

fn read_input(text_before: Option<&str>) -> String {
  match text_before {
    Some(value) => {
      print!("{}", value);
      flush_output();
    },
    None => ()
  }

  let mut data = String::new();

  io::stdin()
    .read_line(&mut data)
    .expect("Failed to read line.")
  ;

  return String::from(data.trim());
}
