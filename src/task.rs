use std::fs::{self, File};
use std::io::{BufReader, BufWriter, BufRead, Write};
use std::path::PathBuf;
use std::io;

pub struct Task {
    pub description: String,
}

pub fn load_tasks(file_path: &PathBuf) -> Result<Vec<Task>, io::Error> {
    if !file_path.exists() {
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        return Ok(Vec::new());
    }

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut tasks = Vec::new();
    for line in reader.lines() {
        let description = line?.trim().to_string();
        if !description.is_empty() {
            tasks.push(Task { description });
        }
    }

    Ok(tasks)
}

pub fn save_tasks(file_path: &PathBuf, tasks: &[Task]) -> Result<(), io::Error> {
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    for task in tasks {
        writeln!(writer, "{}", task.description)?;
    }

    Ok(())
}


pub fn add_task(tasks: &mut Vec<Task>, description: &str) {
    tasks.push(Task { description: description.to_string() });
}

pub fn delete_task(tasks: &mut Vec<Task>, index: usize) -> Result<(), String> {
    if index > 0 && index <= tasks.len() {
        tasks.remove(index - 1);
        Ok(())
    } else {
        Err(format!("Index {} out of bounds.", index))
    }
}

pub fn list_tasks(tasks: &Vec<Task>) {
    for (i, task) in tasks.iter().enumerate() {
        println!("{}: {}", i + 1, task.description);
    }
}