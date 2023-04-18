mod todo_list;
mod todo_item;

use std::io;
use std::io::Write;

fn main() {
    let mut todo_list = todo_list::TodoList::new();

    loop {
        println!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "quit" => break,
            "list" => todo_list.print(),
            "done" => {
                println!("Enter item number to mark as done: ");
                io::stdout().flush().unwrap();
                let mut item_index = String::new();
                io::stdin().read_line(&mut item_index).unwrap();
                let item_index = item_index.trim().parse::<usize>().unwrap();
                todo_list.mark_done(item_index - 1)
            }
            _ => {
                let new_item = todo_item::TodoItem::new(input.to_string());
                todo_list.add_item(new_item);
            }
        }

    }


}
