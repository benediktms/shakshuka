<script lang="ts">
  import Column from './Column.svelte';
  import type { TaskColumn, Task } from '$lib/components/TaskBoard/types';

  export let columns: TaskColumn[];
  export let updateCallback: (tasks: TaskColumn[]) => void;

  function handleDropTaskOnColumn(columnIdx: number, newItems: Task[]) {
    columns[columnIdx].items = newItems.map(t => ({ ...t, status: columns[columnIdx].name }));
    updateCallback([...columns]);
  }
</script>

<section id="task-board" class="h-full">
  <div class="grid-gap-5 grid h-full w-full grid-cols-1 grid-rows-1 p-3 lg:grid-cols-3">
    {#each columns as column, idx (column.id)}
      <Column
        columnId={column.id}
        class="h-full w-full"
        name={column.name}
        tasks={column.items}
        onDrop={newItems => handleDropTaskOnColumn(idx, newItems)}
      />
    {/each}
  </div>
</section>
