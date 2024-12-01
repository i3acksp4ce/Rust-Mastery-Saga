fn main() {
    let mut task = todo_task("task 1", 10);
    println!("{}", task);

    task = todo_task("task 2", 20);
    println!("{}", task);

    task = todo_task("task 3", 30);
    println!("{}", task);
}

fn todo_task(task: &str, time: i32) -> String {
    format!("Task: {} Time: {}", task, time)
}
