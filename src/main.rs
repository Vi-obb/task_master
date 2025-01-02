use task::Task;

mod task;

fn main() {
    let task = Task {
        description: String::from("Complete the task manager project"),
    };
    println!("Task: {}", task.description);
}
