<script lang="ts">
  import { flip } from 'svelte/animate';
  import { dndzone, type DndEvent } from 'svelte-dnd-action';
  import Column from './Column.svelte';
  import type { TaskColumn, Task } from '$lib/types';
  const flipDurationMs = 300;

  export let columns: TaskColumn[];
  export let onFinalUpdate: (tasks: TaskColumn[]) => void;

  function handleDndConsiderColumns(e: CustomEvent<DndEvent<TaskColumn>>) {
    columns = e.detail.items;
  }
  function handleDndFinalizeColumns(e: CustomEvent<DndEvent<TaskColumn>>) {
    onFinalUpdate(e.detail.items);
  }
  function handleItemFinalize(columnIdx: number, newItems: Task[]) {
    columns[columnIdx].items = newItems;
    onFinalUpdate([...columns]);
  }
</script>

<section
  use:dndzone={{ items: columns, flipDurationMs, type: 'column' }}
  on:consider={handleDndConsiderColumns}
  on:finalize={handleDndFinalizeColumns}
>
  <div class="grid-gap-5 grid h-full w-full grid-cols-1 p-3 lg:grid-cols-3">
    {#each columns as { id, name, items }, idx (id)}
      <div animate:flip={{ duration: flipDurationMs }}>
        <Column
          class="w-full"
          {name}
          {items}
          onDrop={newItems => handleItemFinalize(idx, newItems)}
        />
      </div>
    {/each}
  </div>
</section>
