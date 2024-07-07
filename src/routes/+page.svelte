<script lang="ts">
  import Board from '$lib/components/TaskBoard/Board.svelte';
  import { type Task, type TaskColumn, Status } from '$lib/components/TaskBoard/types';
  import { cn } from '$lib/utils';
  import type { HTMLAttributes } from 'svelte/elements';

  let className: HTMLAttributes<HTMLDivElement>['class'] = undefined;
  export { className as class };

  const tasks = Array.from(new Array(24)).map<Task>((e, i) => ({
    id: `TASK-${i}`,
    title: `Task ${i}`,
    description:
      i % 3 === 0
        ? 'Lorem ipsum dolor sit amet consectetur adipisicing elit. Ipsam dignissimos vitae quos excepturi laboriosam ut consequatur sed dolorem amet reiciendis eos quas voluptas voluptatem dicta, minima, corporis soluta libero sunt!'
        : 'Lorem ipsum dolor sit amet consectetur adipisicing elit.',
    status: i < 8 ? Status.TODO : i < 16 && i >= 8 ? Status.IN_PROGRESS : Status.DONE
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

<Board class={cn(className, 'h-full pt-[38px]')} {columns} updateCallback={handleBoardUpdated} />
