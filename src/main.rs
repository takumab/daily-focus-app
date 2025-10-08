use daily_focus_app::*;
use std::io;

fn main() {
    println!("Welcome to Daily Focus APP");
    println!("Please enter a task:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut in_memory_task_repo = InMemoryTaskRepository::new();
    let input = input.trim().to_string();
    create_task(input, &mut in_memory_task_repo).unwrap();

    let tasks = get_all_tasks(&in_memory_task_repo).unwrap();
    println!("All tasks: {:?}", tasks);
}
