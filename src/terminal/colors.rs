pub fn bright_cyan(text: &str) -> String {
  return "\x1b[96m".to_owned() + text + "\x1b[0m";
}

pub fn bright_yellow(text: &str) -> String {
  return "\x1b[93m".to_owned() + text + "\x1b[0m";
}
