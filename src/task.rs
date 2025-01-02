use std::fs::File;
use std::io::{BufReader, BufWriter, BufRead, Write};
use std::path::Path;
use std::io;

pub struct Task {
    pub description: String,
}

pub fn load_tasks(file_path: &str) -> Result<Vec<Task>, io::Error> {
    if !Path::new(file_path).exists() {
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

pub fn save_tasks(file_path: &str, tasks: &[Task]) -> Result<(), io::Error> {
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