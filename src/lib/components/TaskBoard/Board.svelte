<script lang="ts">
  import Column from './Column.svelte';
  import type { Task } from '$lib/components/TaskBoard/types';
  import Resizable from '$ui/resizable';
  import { currentlyFocusedTaskId } from './taskDetailsStore';
  import TaskDetails from './TaskDetails.svelte';
  import type { HTMLAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils';
  import { tasksColumns } from '$lib/globalStores/tasksStore';

  let className: HTMLAttributes<HTMLDivElement>['class'] = undefined;
  export { className as class };

  function handleDropTaskOnColumn(columnIdx: number, newItems: Task[]) {
    $tasksColumns[columnIdx].items = newItems.map(t => ({
      ...t,
      status: $tasksColumns[columnIdx].name
    }));
  }

  function findTaskForExpansion(taskId: string) {
    return $tasksColumns.flatMap(c => c.items).filter(task => task.id === taskId)[0];
  }
</script>

<section id="task-board" class={cn(className)}>
  <Resizable.PaneGroup direction="horizontal" autoSaveId="task-board-panes">
    <Resizable.Pane minSize={65}>
      <!-- TODO: create a proper single column view for small screens -->
      <div class="grid h-full w-full grid-cols-1 px-2 lg:grid-cols-3">
        {#each $tasksColumns as column, idx (column.id)}
          <Column
            columnId={column.id}
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
