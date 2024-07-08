<script lang="ts">
  import { flip } from 'svelte/animate';
  import { dndzone, type DndEvent } from 'svelte-dnd-action';
  import { Status, type Task, type TaskColumnId } from './types';
  import TaskCard from '$lib/components/TaskBoard/TaskCard.svelte';
  import type { HTMLAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils';
  import { ScrollArea } from '$ui/scroll-area';

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

<div id={columnId.toUpperCase()} class={cn(className, 'flex flex-col overflow-y-hidden')}>
  <h2 class="text-center text-lg">{columnNameMap[name]}</h2>
  <ScrollArea>
    <div
      class="m-2 h-full pr-2"
      use:dndzone={{
        items: tasks,
        flipDurationMs
      }}
      on:consider={onConsider}
      on:finalize={e => onDrop(e.detail.items)}
    >
      {#each tasks as task (task.id)}
        <div animate:flip={{ duration: flipDurationMs }}>
          <TaskCard class="mb-3 w-full" {task} />
        </div>
      {/each}
    </div>
  </ScrollArea>
</div>
