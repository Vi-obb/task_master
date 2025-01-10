mod task;

use clap::{Parser, Subcommand};
use dirs::home_dir;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        description: String,
    },
    Delete {
        index: usize,
    },
    Edit {
        index: usize,
        description: String,
    },
    List,
}

fn get_database_path() -> PathBuf {
    let mut config_dir = home_dir().expect("Failed to find home directory");
    config_dir.push(".taskmstr");
    std::fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    config_dir.push("tasks.db");
    config_dir
}

fn main() {
    let cli = Cli::parse();
    let db_path = get_database_path();
    let conn = task::init_database(&db_path)
        .expect("Failed to initialize database");

    match &cli.command {
        Commands::Add { description } => {
            if let Err(e) = task::add_task(&conn, description) {
                eprintln!("Error adding task: {}", e);
                return;
            }
            println!("Task added successfully: {}", description);
        }
        Commands::Delete { index } => {
            if let Err(e) = task::delete_task(&conn, *index as i64) {
                eprintln!("Error deleting task: {}", e);
                return;
            }
            println!("Task {} deleted successfully", index);
        }
        Commands::Edit { index, description } => {
            if let Err(e) = task::edit_task(&conn, *index as i64, description) {
                eprintln!("Error editing task: {}", e);
                return;
            }
            println!("Task {} edited successfully", index);
        }
        Commands::List => {
            match task::load_tasks(&conn) {
                Ok(tasks) => {
                    if tasks.is_empty() {
                        println!("No tasks to display.");
                    } else {
                        task::list_tasks(&tasks);
                    }
                }
                Err(e) => eprintln!("Error loading tasks: {}", e),
            }
        }
    }
}
