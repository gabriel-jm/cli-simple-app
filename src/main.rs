use std::io::{self, Write};

fn main() {
    clear();
    welcome();
    io::stdout().flush().expect("Unexpected error on read line");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line.")
    ;

    println!("Input: [{}]", name);

    if name.trim().eq("signin") {
        clear();
        println!("Sign in!!!")
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
    std::process::Command::new("clear").status().unwrap();
}
