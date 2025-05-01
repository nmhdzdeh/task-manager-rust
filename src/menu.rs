use crate::storage::save_tasks_to_file;
use crate::task::Task;
use std::io::{self, Write};

pub fn add_task(tasks: &mut Vec<Task>) {
    let title = read_input("Enter task title: ");
    let desc = read_input("Enter task description:");
    tasks.push(Task::create_task(&title, &desc));
    save_tasks_to_file(&tasks, "tasks.json").expect("Failed to save tasks");
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn complete_task(tasks: &mut Vec<Task>) {
    loop {
        println!("Which task would you like to mark as completed?");
        tasks.sort_by_key(|task| task.completed);

        for (index, task) in tasks.iter().enumerate() {
            println!("{}. {}", index + 1, task);
        }
        let task_number = read_input("Enter task number or type 'exit' to go back:");
        if task_number == "exit" {
            break;
        }
        if let Ok(index) = task_number.parse::<usize>() {
            if index > 0 && index <= tasks.len() {
                tasks[index - 1].complete();
                save_tasks_to_file(&tasks, "tasks.json").expect("Failed to save tasks");

                println!("Task completed!");
            } else {
                println!("Invalid task number!");
            }
        } else {
            println!("Please enter a valid number!");
        }
    }
}

pub fn remove_task(tasks: &mut Vec<Task>) {
    loop {
        println!("Which task would you like to remove?");
        tasks.sort_by_key(|task| task.completed); // مرتب‌سازی بر اساس تکمیل بودن تسک

        // نمایش تسک‌ها
        let display_list: Vec<String> = tasks
            .iter()
            .enumerate()
            .map(|(index, task)| format!("{}. {}", index + 1, task))
            .collect();

        for line in display_list {
            println!("{}", line);
        }
        let task_number = read_input("Enter task number or type 'exit' to go back:");
        if task_number == "exit" {
            break;
        }
        if let Ok(index) = task_number.parse::<usize>() {
            if index > 0 && index <= tasks.len() {
                tasks.remove(index - 1);
                save_tasks_to_file(&tasks, "tasks.json").expect("Failed to save tasks");

                println!("Task removed!");
            } else {
                println!("Invalid task number!");
            }
        } else {
            println!("Please enter a valid number!");
        }
    }
}

pub fn edit_task(tasks: &mut Vec<Task>) {
    loop {
        println!("Which task would you like to edit?");
        tasks.sort_by_key(|task| task.completed);

        for (index, task) in tasks.iter().enumerate() {
            println!("{}. {}", index + 1, task);
        }
        let task_number = read_input("Enter task number or type 'exit' to go back:");
        if task_number == "exit" {
            break;
        }
        if let Ok(index) = task_number.parse::<usize>() {
            if index > 0 && index <= tasks.len() {
                let task = &mut tasks[index - 1];
                println!("What would you like to edit?");
                println!("1. Title\n2. Description\n3. Both");

                let choice = read_input("Enter your choice:");
                match choice.as_str() {
                    "1" => {
                        let new_title = read_input("Enter new title:");
                        task.title = new_title;
                    }
                    "2" => {
                        let new_desc = read_input("Enter new description:");
                        task.desc = new_desc;
                    }
                    "3" => {
                        let new_title = read_input("Enter new title:");
                        let new_desc = read_input("Enter new description:");
                        task.title = new_title;
                        task.desc = new_desc;
                    }
                    _ => {
                        println!("Invalid choice!");
                        continue;
                    }
                }

                save_tasks_to_file(tasks, "tasks.json").expect("Failed to save tasks");
                println!("Task updated!");
            } else {
                println!("Invalid task number!");
            }
        } else {
            println!("Please enter a valid number!");
        }

        let again = read_input("Edit another task? (yes/no):");
        if again.to_lowercase() != "yes" {
            break;
        }
    }
}

pub fn filter_tasks(tasks: &Vec<Task>) {
  loop {
      println!("\nFilter Options:");
      println!("1. Show only incomplete tasks ⬜");
      println!("2. Show only completed tasks ✅");
      println!("3. Show all tasks");
      println!("4. Back to main menu");

      let choice = read_input("Choose an option:");

      match choice.as_str() {
          "1" => {
              println!("\nIncomplete Tasks:");
              for (i, task) in tasks.iter().filter(|t| !t.completed).enumerate() {
                  println!("{}. {}", i + 1, task);
              }
          }
          "2" => {
              println!("\nCompleted Tasks:");
              for (i, task) in tasks.iter().filter(|t| t.completed).enumerate() {
                  println!("{}. {}", i + 1, task);
              }
          }
          "3" => {
              println!("\nAll Tasks:");
              for (i, task) in tasks.iter().enumerate() {
                  println!("{}. {}", i + 1, task);
              }
          }
          "4" => break,
          _ => println!("Invalid option. Please choose again."),
      }
  }
}
