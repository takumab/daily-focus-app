use mockall::automock;
use std::{collections::HashMap, time::SystemTime};
use uuid::{Uuid, uuid};

#[derive(Debug, PartialEq, Clone)]
enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, PartialEq)]
pub enum SaveError {
    DuplicateId,
    NotFound,
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
    fn get_by_id(&self, id: UUID) -> Result<Task, SaveError>;
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

    fn get_by_id(&self, id: UUID) -> Result<Task, SaveError> {
        self.store.get(&id).cloned().ok_or(SaveError::NotFound)
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

fn get_task_by(id: UUID, repo: &impl TaskRepository) -> Result<Task, SaveError> {
    let task = repo.get_by_id(id)?;
    print!("Task: {:?}", task);
    Ok(task)
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

    #[test]
    fn it_should_list_multiple_tasks() {
        let input = "Another task".to_string();
        let input_two = "A simple task".to_string();
        let task = Task {
            id: UUID(Uuid::new_v4()),
            title: input,
            status: Status::Todo,
            created_at: SystemTime::now(),
        };
        let another_task = Task {
            id: UUID(Uuid::new_v4()),
            title: input_two,
            status: Status::InProgress,
            created_at: SystemTime::now(),
        };
        let tasks = vec![task.clone(), another_task.clone()];
        let expected_tasks = tasks.clone();

        let mut in_memory_task_repo_mock = MockTaskRepository::new();
        in_memory_task_repo_mock
            .expect_get()
            .times(1)
            .returning(move || Ok(tasks.clone()));

        let result = get_all_tasks(&in_memory_task_repo_mock);

        assert_eq!(result, Ok(expected_tasks));
    }

    #[test]
    fn it_should_get_a_specific_task() {
        let input = "Another task".to_string();
        let task = Task {
            id: UUID(Uuid::new_v4()),
            title: input,
            status: Status::Todo,
            created_at: SystemTime::now(),
        };
        let task_id = UUID(uuid!("123e4567-e89b-12d3-a456-426614174000"));
        let expected_task = task.clone();

        let mut in_memory_task_repo_mock = MockTaskRepository::new();
        in_memory_task_repo_mock
            .expect_get_by_id()
            .withf({
                let task_id = task_id.clone();
                move |id: &UUID| *id == task_id
            })
            .times(1)
            .returning({
                let task = task.clone();
                move |_id: UUID| Ok(task.clone())
            });

        let result = get_task_by(task_id, &in_memory_task_repo_mock);

        assert_eq!(result, Ok(expected_task));
    }
}
