mod menu;
mod storage;
mod task;

use menu::{edit_task, filter_tasks};

use crate::menu::{add_task, complete_task, remove_task};
use crate::storage::load_tasks_from_file;

fn main() {
    let mut tasks = load_tasks_from_file("tasks.json");

    loop {
        println!("\nChoose: [add] [complete] [remove] [edit] [filter] [exit]");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "add" => add_task(&mut tasks),
            "complete" => complete_task(&mut tasks),
            "remove" => remove_task(&mut tasks),
            "edit" => edit_task(&mut tasks),
            "filter" => filter_tasks(&tasks),
            "exit" => break,
            _ => println!("Unknown command!"),
        }
    }
}
