use std::{io::{self, Write, Read}, process, thread, time};

pub fn clear() {
  process::Command::new("clear")
    .status()
    .unwrap();
}

pub fn flush_output() {
  io::stdout()
    .flush()
    .expect("Unexpected error on read line");
}

pub fn read_input(text_before: Option<&str>) -> String {
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

pub fn pause() {
  let mut stdin = io::stdin();
  let mut stdout = io::stdout();

  write!(stdout, "Press ENTER to continue...").unwrap();
  stdout.flush().unwrap();

  stdin.read(&mut [0u8]).unwrap();
}

pub fn sleep(milliseconds: u64) {
  thread::sleep(
    time::Duration::from_millis(milliseconds)
  );
}
