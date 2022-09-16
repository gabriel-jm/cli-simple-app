use colored::Colorize;

use crate::state::Account;

pub fn header(title: &str, account: &Option<Account>) {
  print!("{}", format!("🔹 {}", title).cyan());
  
  match account {
    Some(acc) => println!("\t\t\t[{}]", acc.name.green()),
    None => println!("{}", "\t\t\tNot logged".black())
  };
}