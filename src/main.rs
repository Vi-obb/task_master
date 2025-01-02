mod task;

use std::env;
use dirs::home_dir;
use std::path::PathBuf;

fn get_tasks_file() -> PathBuf {
    let mut config_dir = home_dir().expect("Failed to find home directory");
    config_dir.push(".taskmstr");
    config_dir.push("tasks.txt");
    config_dir
}

fn main() {
    let tasks_file = get_tasks_file();
    let mut tasks = match task::load_tasks(&tasks_file) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error loading tasks: {}", e);
            Vec::new()
        }
    };

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: taskmstr [add|delete|list] [args...]");
        return;
    }

    let action = &args[1];

    match action.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: taskmstr add [description]");
                return;
            }
            let description = &args[2];
            task::add_task(&mut tasks, description);
            println!("Adding task: {}", description);
        }
        "delete" => {
            if args.len() < 3 {
                println!("Usage: taskmstr delete [index]");
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
            println!("Usage: taskmstr [add|delete|list] [args...]");
            return;
        }
    }

    if let Err(e) = task::save_tasks(&tasks_file, &tasks) {
        eprintln!("Error saving tasks: {}", e);
    } else {
        println!("Tasks saved successfully.");
    }
}
