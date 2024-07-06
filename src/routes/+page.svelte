<script lang="ts">
  import Board from '$lib/components/TaskBoard/Board.svelte';
  import { type Task, type TaskColumn, Status } from '$lib/components/TaskBoard/types';

  const tasks: Array<Task> = [
    {
      id: 'TASK-1',
      title: 'Task',
      description:
        'Lorem ipsum dolor sit amet consectetur adipisicing elit. Ipsam dignissimos vitae quos excepturi laboriosam ut consequatur sed dolorem amet reiciendis eos quas voluptas voluptatem dicta, minima, corporis soluta libero sunt!',
      status: Status.TODO
    },
    { id: 'TASK-2', title: 'Task', description: 'Task desciption', status: Status.TODO },
    { id: 'TASK-3', title: 'Task', description: 'Task desciption', status: Status.IN_PROGRESS },
    { id: 'TASK-4', title: 'Task', description: 'Task desciption', status: Status.IN_PROGRESS },
    { id: 'TASK-5', title: 'Task', description: 'Task desciption', status: Status.DONE },
    { id: 'TASK-6', title: 'Task', description: 'Task desciption', status: Status.DONE }
  ];

  let columns: TaskColumn[] = [
    {
      id: 'task-todo-column',
      name: Status.TODO,
      items: tasks.filter(t => t.status === Status.TODO)
    },
    {
      id: 'task-in_progress-column',
      name: Status.IN_PROGRESS,
      items: tasks.filter(t => t.status === Status.IN_PROGRESS)
    },
    {
      id: 'task-done-column',
      name: Status.DONE,
      items: tasks.filter(t => t.status === Status.DONE)
    }
  ];

  function handleBoardUpdated(newColumnsData: TaskColumn[]) {
    columns = newColumnsData;
  }
</script>

<div class="container h-full">
  <Board {columns} updateCallback={handleBoardUpdated} />
</div>
