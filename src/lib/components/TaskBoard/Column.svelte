<script lang="ts">
  import { flip } from 'svelte/animate';
  import { dndzone, type DndEvent } from 'svelte-dnd-action';
  import { Status, type Task, type TaskColumnId } from './types';
  import TaskCard from '$lib/components/TaskBoard/TaskCard.svelte';
  import type { HTMLAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils';

  let className: HTMLAttributes<HTMLDivElement>['class'] = undefined;
  export { className as class };

  export let columnId: TaskColumnId;
  export let name: Status;
  export let tasks: Task[];
  export let onDrop: (items: Task[]) => void;

  const flipDurationMs = 150;

  const columnNameMap: Record<Status, string> = {
    [Status.TODO]: 'To Do',
    [Status.IN_PROGRESS]: 'In Progress',
    [Status.DONE]: 'Done'
  };

  function onConsider(e: CustomEvent<DndEvent<Task>>) {
    tasks = e.detail.items;
  }
</script>

<div id={columnId.toUpperCase()} class={cn(className, 'flex flex-col px-2')}>
  <h2 class="my-4 text-center text-lg">{columnNameMap[name]}</h2>
  <div
    use:dndzone={{
      items: tasks,
      flipDurationMs
    }}
    on:consider={onConsider}
    on:finalize={e => onDrop(e.detail.items)}
  >
    {#each tasks as task (task.id)}
      <span animate:flip={{ duration: flipDurationMs }}>
        <TaskCard class="mb-3 w-full" {task} />
      </span>
    {/each}
  </div>
</div>
