# Daily Focus Plan App
This app's goal is to enable me to create a Daily Focus Plan

## Functionality
- Create task and an approximate time it takes to complete the task
- These task can have subtask, but the subtask can't have subtask at least for the first iteration in order to keep it to one line of indentation
- The subtask will also have an approximate time it takes to complete them
- Ideally these approximate time should sum up the time for the parent task
- The time will be in minutes for now
- Once a task/subtask is complete, it can be labled as done
- All tasks are done once all have been labled as done
- Ideally, once the task has started, a timer starts
- Ideally, once the task(s) are finished, a timer stops, indicating how long the task(s) actually took

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
