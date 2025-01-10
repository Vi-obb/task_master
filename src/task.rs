use rusqlite::{Connection, Result, params};
use std::path::PathBuf;

pub struct Task {
    pub id: i64,
    pub description: String,
}

pub fn init_database(db_path: &PathBuf) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

pub fn load_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT id, description FROM tasks")?;
    let tasks = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
        })
    })?;

    let mut result = Vec::new();
    for task in tasks {
        result.push(task?);
    }
    Ok(result)
}

pub fn add_task(conn: &Connection, description: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (description) VALUES (?)",
        params![description],
    )?;
    Ok(())
}

pub fn delete_task(conn: &Connection, id: i64) -> Result<()> {
    let affected = conn.execute("DELETE FROM tasks WHERE id = ?", params![id])?;
    if affected == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }
    Ok(())
}

pub fn edit_task(conn: &Connection, id: i64, new_description: &str) -> Result<()> {
    let affected = conn.execute(
        "UPDATE tasks SET description = ? WHERE id = ?",
        params![new_description, id],
    )?;
    if affected == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }
    Ok(())
}

pub fn list_tasks(tasks: &[Task]) {
    for task in tasks {
        println!("{}: {}", task.id, task.description);
    }
}