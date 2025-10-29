use daily_focus_app::*;
use std::env::args;
use std::io;
use uuid::uuid;

fn main() {
    println!("Welcome to Daily Focus APP");
    println!("Please enter a task:");

    let args: Vec<String> = args().collect();


    let mut in_memory_task_repo = InMemoryTaskRepository::new();

    let flag = &args[1];
    match flag.as_str() {
        "create-task" => {
            let input = String::new();
            let mut input = input.trim().to_string();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            create_task(input, &mut in_memory_task_repo).unwrap();
            println!("Task has been created");
        }
        "get-tasks" => {
            let input = String::new();
            let mut input = input.trim().to_string();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            create_task(input, &mut in_memory_task_repo).unwrap();
            let tasks = get_all_tasks(&mut in_memory_task_repo).unwrap();
            println!("All tasks {:?}", tasks)
        }
        "update-task" => {
            // TODO: failing because there is no Task with the ID below
            let input = String::new();
            let mut input = input.trim().to_string();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            create_task(input, &mut in_memory_task_repo).unwrap();
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
