<script lang="ts">
  import Board from '$lib/components/Board/Board.svelte';
  import { type Task, type TaskColumn, Status } from '$lib/types';

  const tasks: Array<Task> = [
    {
      id: 1,
      title: 'Task',
      description:
        'Lorem ipsum dolor sit amet consectetur adipisicing elit. Ipsam dignissimos vitae quos excepturi laboriosam ut consequatur sed dolorem amet reiciendis eos quas voluptas voluptatem dicta, minima, corporis soluta libero sunt!',
      status: Status.Todo
    },
    { id: 2, title: 'Task', description: 'Task desciption', status: Status.Todo },
    { id: 3, title: 'Task', description: 'Task desciption', status: Status.InProgress },
    { id: 4, title: 'Task', description: 'Task desciption', status: Status.InProgress },
    { id: 5, title: 'Task', description: 'Task desciption', status: Status.Done },
    { id: 6, title: 'Task', description: 'Task desciption', status: Status.Done }
  ];

  let columns: TaskColumn[] = [
    {
      id: 'todo-column',
      name: Status.Todo,
      items: tasks.filter(t => t.status === Status.Todo)
    },
    {
      id: 'in-progress-column',
      name: Status.InProgress,
      items: tasks.filter(t => t.status === Status.InProgress)
    },
    {
      id: 'done-column',
      name: Status.Done,
      items: tasks.filter(t => t.status === Status.Done)
    }
  ];

  function handleBoardUpdated(newColumnsData: TaskColumn[]) {
    columns = newColumnsData;
  }
</script>

<div class="container h-full">
  <Board {columns} onFinalUpdate={handleBoardUpdated} />
</div>
