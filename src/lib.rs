pub use mockall::*;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, PartialEq)]
pub enum SaveError {
    DuplicateId,
}

#[derive(Debug, PartialEq)]
struct UUID(Uuid);

#[derive(Debug, PartialEq)]
struct Task {
    id: UUID,
    title: String,
    status: Status,
    created_at: SystemTime,
}

#[cfg_attr(test, automock)]
pub trait TaskRepository {
    fn save(&self, task: Task) -> Result<(), SaveError>;
}

pub fn create_task(input: String, repo: &mut impl TaskRepository) -> Result<(), SaveError> {
    let task = Task {
        id: UUID(Uuid::new_v4()),
        title: input,
        status: Status::InProgress,
        created_at: SystemTime::now(),
    };
    let _ = repo.save(task)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_a_task() {
        let input = "Some two minute task".to_string();

        let mut in_memory_task_repo_mock = MockTaskRepository::new();
        in_memory_task_repo_mock
            .expect_save()
            .withf(|task: &Task| task.title == "Some two minute task".to_string())
            .times(1)
            .returning(|_| Ok(()));

        let result = create_task(input, &mut in_memory_task_repo_mock);

        assert_eq!(result, Ok(()));
    }
}
