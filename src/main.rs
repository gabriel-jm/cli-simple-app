use std::{
  io::{self, Write, Read},
  process,
  thread,
  time,
  fs::File,
  path::Path
};

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
    "q" => println!("\nExiting..."),
    _ => not_found()
  }
}

fn sign_in() {
  clear();
  println!("# Sign In\n\n");

  let name = read_input(Some("Enter your name: "));
  let password = read_input(Some("Enter your password: "));

  let data = "{\"name\":\"".to_owned()
    + name.as_ref()
    + "\",\"password\":\""
    + password.as_ref()
    + "\"}"
  ;

  append_to_file("./src/data.json", data.as_ref());

  home();
}

fn welcome() {
  print_multi_lines(vec![
    "# Home\n",
    "Welcome to the store",
    "\n\tYou're not current loged",
    "\nCommands:",
    " signin - Create an account",
    " login - Log in an existing account",
    " q - Exit application"
  ]);

  print!("\n> ");
}

fn not_found() {
  println!("\x1b[93m\n** Command not found **\n\x1b[0m");
  sleep(500);
  pause();
  home();
}

fn pause() {
  let mut stdin = io::stdin();
  let mut stdout = io::stdout();

  write!(stdout, "Press ENTER to continue...").unwrap();
  stdout.flush().unwrap();

  stdin.read(&mut [0u8]).unwrap();
}

fn print_multi_lines(texts: Vec<&str>) {
  for text in texts {
    println!("{}", text);
  }
}

fn clear() {
  process::Command::new("clear")
    .status()
    .unwrap();
}

fn sleep(milliseconds: u64) {
  thread::sleep(
    time::Duration::from_millis(milliseconds)
  );
}

fn flush_output() {
  io::stdout()
    .flush()
    .expect("Unexpected error on read line");
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

fn append_to_file(path: &str, data: &str) {
  if !Path::new(path).exists() {
    File::create(path)
      .expect("Unable to create file")
      .write(String::from("[]").as_ref())
      .expect("Unable to create file")
    ;
  }

  let mut from_file = String::new();

  File::options()
    .read(true)
    .open(path)
    .expect("Unable to create new user")
    .read_to_string(&mut from_file)
    .expect("Unable to create new user")
  ;

  let new_data = 
    if from_file.len() == 2 {
      "[".to_owned() + data.as_ref() + "]"
    } else {
      from_file.replace("]", ",") + data.as_ref() + "]"
    }
  ;

  File::options()
    .write(true)
    .open(path)
    .expect("Unable to create new user")
    .write(new_data.as_ref())
    .expect("Unable to create new user")
  ;
}
