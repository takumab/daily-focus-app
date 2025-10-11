# Daily Focus Plan App
This app's goal is to enable me to create a Daily Focus Plan

## New Functionality
- [X] Create a new task
  - [X] save the task
- [X] list tasks
- [ ] Update a task
  - [ ] Mark a task as done
  - [X] Update title of task
- [ ] Delete a task

## Light Sketch

### One task
```sh
daily-focus-plan start
> Hello, Takuma! Today is [Date] and [Time], what would you like to focus on today?
> Some example task
> Estimated time to complete task?
> 20 minutes
> Good!
> Any other tasks in mind?
> No
> Ok
> Your task are: "Some task" that will take approximately 20 minutes
```

### Multiple tasks
```sh
daily-focus-plan start
> Hello, Takuma! Today is [Date] and [Time], what would you like to focus on today?
> Some example task
> Estimated time to complete task?
> 20 minutes
> Any subtask?
> No
> Any other tasks?
> Yes
> Enter task now please
> Another task
> Estimated time to complete task?
> 10 minutes
> Any subtask?
> No
> Any other tasks in mind?
> No
> Good!
> Your tasks are: "Some task" that will take approximately 20 minutes and "Another task" that will take approximately 10 minutes
> Cheers
> No worries
```
