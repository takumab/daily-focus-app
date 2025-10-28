use daily_focus_app::*;
use std::env::args;
use std::io;
use uuid::uuid;

fn main() {
    println!("Welcome to Daily Focus APP");
    println!("Please enter a task:");

    let args: Vec<String> = args().collect();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");


    let mut in_memory_task_repo = InMemoryTaskRepository::new();
    let input = input.trim().to_string();

    // TODO: rethink logic here
    let flag = &args[1];
    match flag.as_str() {
        "create-task" => {
            create_task(input, &mut in_memory_task_repo).unwrap();
            println!("Task has been created");
        }
        "get-tasks" => {
            println!("In get tasks branch");
            let tasks = get_all_tasks(&mut in_memory_task_repo).unwrap();
            println!("All tasks {:?}", tasks)
        }
        "update-task" => {
            let task = update_task(
                UUID(uuid!("123e4567-e89b-12d3-a456-426614174000")),
                &mut in_memory_task_repo, Some("a title".to_string()),
                Some(Status::InProgress),
            ).unwrap();
            println!("Task with id {:?} updated", task)
        }
        _ => println!("Command not found")
    }
}
