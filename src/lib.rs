use mockall::automock;
use std::{collections::HashMap, time::SystemTime};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, PartialEq)]
pub enum SaveError {
    DuplicateId,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct UUID(Uuid);

#[derive(Debug, PartialEq, Clone)]
struct Task {
    id: UUID,
    title: String,
    status: Status,
    created_at: SystemTime,
}

#[cfg_attr(test, automock)]
trait TaskRepository {
    fn save(&mut self, task: Task) -> Result<(), SaveError>;
    fn get(&self) -> Result<Vec<Task>, SaveError>;
}

struct InMemoryTaskRepository {
    store: HashMap<UUID, Task>,
}

impl TaskRepository for InMemoryTaskRepository {
    fn save(&mut self, task: Task) -> Result<(), SaveError> {
        if self.store.contains_key(&task.id) {
            return Err(SaveError::DuplicateId);
        }
        self.store.insert(task.id.clone(), task);
        Ok(())
    }

    fn get(&self) -> Result<Vec<Task>, SaveError> {
        Ok(self.store.values().cloned().collect())
    }
}

fn create_task(input: String, repo: &mut impl TaskRepository) -> Result<(), SaveError> {
    let task = Task {
        id: UUID(Uuid::new_v4()),
        title: input,
        status: Status::Todo,
        created_at: SystemTime::now(),
    };
    repo.save(task)?;
    Ok(())
}

fn get_all_tasks(repo: &impl TaskRepository) -> Result<Vec<Task>, SaveError> {
    let tasks = repo.get()?;
    Ok(tasks)
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
            .withf(|task: &Task| task.title == "Some two minute task")
            .times(1)
            .returning(|_| Ok(()));

        let result = create_task(input, &mut in_memory_task_repo_mock);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn it_should_create_another_task() {
        let input = "Another task".to_string();

        let mut in_memory_task_repo_mock = MockTaskRepository::new();
        in_memory_task_repo_mock
            .expect_save()
            .withf(|task: &Task| task.title == "Another task")
            .times(1)
            .returning(|_| Ok(()));

        let result = create_task(input, &mut in_memory_task_repo_mock);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn it_should_list_tasks() {
        let input = "Another task".to_string();
        let task = Task {
            id: UUID(Uuid::new_v4()),
            title: input,
            status: Status::Todo,
            created_at: SystemTime::now(),
        };
        let tasks = vec![task.clone()];
        let expected_tasks = tasks.clone();

        let mut in_memory_task_repo_mock = MockTaskRepository::new();
        in_memory_task_repo_mock
            .expect_get()
            .times(1)
            .returning(move || Ok(tasks.clone()));

        let result = get_all_tasks(&in_memory_task_repo_mock);

        assert_eq!(result, Ok(expected_tasks));
    }
}
