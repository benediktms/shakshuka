<script lang="ts">
  import Board from '$lib/components/TaskBoard/Board.svelte';
  import { type Task, type TaskColumn, Status } from '$lib/components/TaskBoard/types';

  const tasks = Array.from(new Array(12)).map<Task>((e, i) => ({
    id: `TASK-${i}`,
    title: `Task ${i}`,
    description:
      'Lorem ipsum dolor sit amet consectetur adipisicing elit. Ipsam dignissimos vitae quos excepturi laboriosam ut consequatur sed dolorem amet reiciendis eos quas voluptas voluptatem dicta, minima, corporis soluta libero sunt!',
    status: i < 4 ? Status.TODO : i < 8 && i >= 4 ? Status.IN_PROGRESS : Status.DONE
  }));

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
