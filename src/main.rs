mod task;

use std::env;

const TASKS_FILE: &str = "tasks.txt";

fn main() {
    let mut tasks = match task::load_tasks(TASKS_FILE) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error loading tasks: {}", e);
            Vec::new()
        }
    };

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: task_master [add|delete|list] [args...]");
        return;
    }

    let action = &args[1];

    match action.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: task_master add [description]");
                return;
            }
            let description = &args[2];
            task::add_task(&mut tasks, description);
            println!("Adding task: {}", description);
        }
        "delete" => {
            if args.len() < 3 {
                println!("Usage: task_master delete [index]");
                return;
            }
            let index = match args[2].parse::<usize>() {
                Ok(idx) => idx,
                Err(_) => {
                    println!("Invalid index provided.");
                    return;
                }
            };
            match task::delete_task(&mut tasks, index) {
                Ok(_) => println!("Deleting task {}", index),
                Err(e) => println!("Error deleting task: {}", e),
            }
        }
        "list" => {
            if tasks.is_empty() {
                println!("No tasks to display.");
            } else {
                task::list_tasks(&tasks);
            }
        }
        _ => {
            println!("Unknown command: {}", action);
            println!("Usage: task_master [add|delete|list] [args...]");
            return;
        }
    }

    if let Err(e) = task::save_tasks(TASKS_FILE, &tasks) {
        eprintln!("Error saving tasks: {}", e);
    } else {
        println!("Tasks saved successfully.");
    }
}
