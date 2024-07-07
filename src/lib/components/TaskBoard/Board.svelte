<script lang="ts">
  import Column from './Column.svelte';
  import type { TaskColumn, Task } from '$lib/components/TaskBoard/types';
  import Resizable from '$ui/resizable';
  import { currentlyFocusedTaskId } from './taskDetailsStore';
  import TaskDetails from './TaskDetails.svelte';

  export let columns: TaskColumn[];
  export let updateCallback: (tasks: TaskColumn[]) => void;

  function handleDropTaskOnColumn(columnIdx: number, newItems: Task[]) {
    columns[columnIdx].items = newItems.map(t => ({ ...t, status: columns[columnIdx].name }));
    updateCallback([...columns]);
  }

  function findTaskForExpansion(taskId: string) {
    return columns.flatMap(c => c.items).filter(task => task.id === taskId)[0];
  }
</script>

<section id="task-board" class="h-full">
  <Resizable.PaneGroup direction="horizontal">
    <Resizable.Pane>
      <div class="grid h-full w-full grid-cols-1 p-3 lg:grid-cols-3">
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
    </Resizable.Pane>
    {#if !!$currentlyFocusedTaskId}
      <Resizable.Handle />
    {/if}
    {#if !!$currentlyFocusedTaskId}
      <Resizable.Pane
        defaultSize={33}
        collapsible
        collapsedSize={0}
        minSize={25}
        onCollapse={() => ($currentlyFocusedTaskId = undefined)}
      >
        <TaskDetails task={findTaskForExpansion($currentlyFocusedTaskId)} />
      </Resizable.Pane>
    {/if}
  </Resizable.PaneGroup>
</section>
