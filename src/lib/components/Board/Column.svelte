<script lang="ts">
  import { flip } from 'svelte/animate';
  import { dndzone, type DndEvent } from 'svelte-dnd-action';
  import type { Status, Task } from '../../types';
  import TaskCard from '$lib/components/TaskCard.svelte';
  import type { HTMLAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils';

  let className: HTMLAttributes<HTMLDivElement>['class'] = undefined;
  export { className as class };

  export let name: Status;
  export let items: Task[];
  export let onDrop: (item: Task[]) => void;

  const flipDurationMs = 150;

  function handleDndConsiderCards(e: CustomEvent<DndEvent<Task>>) {
    console.warn('got consider', name);
    items = e.detail.items;
  }
  function handleDndFinalizeCards(e: CustomEvent<DndEvent<Task>>) {
    onDrop(e.detail.items);
  }
</script>

<div class={cn(className, 'flex flex-col px-2')}>
  <h2 class="my-4 text-center text-lg">{name}</h2>
  <div
    use:dndzone={{ items, flipDurationMs, zoneTabIndex: -1 }}
    on:consider={handleDndConsiderCards}
    on:finalize={handleDndFinalizeCards}
  >
    {#each items as task (task.id)}
      <div animate:flip={{ duration: flipDurationMs }}>
        <TaskCard class="mb-3 w-full" {task}>
          <div slot="footer">foo</div>
        </TaskCard>
      </div>
    {/each}
  </div>
</div>
