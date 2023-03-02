use colored::Colorize;

use crate::{state::{Account, ToDoItem}, terminal::in_out::{clear, read_input}};

use super::components::header;

pub struct CreateToDoItemProps<'a> {
    pub account: &'a Option<Account>,
    pub list_title: &'a str,
    pub items: &'a Vec<ToDoItem>
}

pub fn create_todo_item(props: CreateToDoItemProps) -> Vec<ToDoItem> {
    let list_title = props.list_title;
    clear();
    header(
        format!("Create ToDo Item for {list_title}").as_str(),
        props.account
    );

    println!("{}", "\nEnter 'q' in any field to cancel".bright_black());

    let name = read_input(Some("\nType what to do: "));

    let to_do_item = ToDoItem {
        name,
        checked: false
    };

    let mut items = Vec::from(&props.items[..]);
    items.push(to_do_item);

    return items;
}
