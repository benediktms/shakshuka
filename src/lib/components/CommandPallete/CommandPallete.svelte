<script lang="ts">
  import Check from 'lucide-svelte/icons/check';
  import { onDestroy, tick } from 'svelte';
  import { Button } from '$lib/components/ui/button/index.js';
  import { cn } from '$lib/utils.js';
  import Popover from '$ui/popover';
  import Command from '$ui/command';
  import { commandPallete } from './commandPalleteStore';
  import { allTasks } from '$lib/globalStores/tasksStore';
  import { Search } from 'lucide-svelte';
  import { currentlyFocusedTaskId } from '../TaskBoard/taskDetailsStore';
  import ScrollArea from '$ui/scroll-area/scroll-area.svelte';

  let open = false;
  let selectedTask: string | undefined = undefined;

  const componentWidth = 'min-w-[360px]';

  // We want to refocus the trigger button when the user selects
  // an item from the list so users can continue navigating the
  // rest of the form with the keyboard.
  function closeAndFocusTrigger(triggerId: string) {
    open = false;
    selectedTask = undefined;
    tick().then(() => {
      document.getElementById(triggerId)?.focus();
    });
  }

  const unsubscribe = commandPallete.subscribe(state => {
    open = state.open;
    selectedTask = state.value;
  });

  onDestroy(() => {
    unsubscribe();
  });
</script>

<Popover.Root bind:open let:ids>
  <Popover.Trigger asChild let:builder>
    <Button
      builders={[builder]}
      variant="outline"
      role="combobox"
      aria-expanded={open}
      class={cn('h-[23px] justify-around hover:border-primary', componentWidth)}
    >
      <span class="flex items-center">
        <Search class="mr-2 h-4 w-4 text-primary" />
        <p>Tasks</p>
      </span>
    </Button>
  </Popover.Trigger>
  <Popover.Content class={cn('mt-2 p-0', componentWidth)}>
    <Command.Root>
      <Command.Input placeholder="Search tasks..." />
      <Command.Empty>No framework found.</Command.Empty>
      <ScrollArea class="h-80">
        <Command.Group>
          {#each $allTasks as task}
            <Command.Item
              value={task.id}
              onSelect={taskId => {
                selectedTask = taskId;
                $currentlyFocusedTaskId = selectedTask;
                closeAndFocusTrigger(ids.trigger);
              }}
            >
              <Check
                class={cn(
                  'mr-2 h-4 w-4',
                  $currentlyFocusedTaskId !== task.id && 'text-transparent'
                )}
              />
              {task.title}
            </Command.Item>
          {/each}
        </Command.Group>
      </ScrollArea>
    </Command.Root>
  </Popover.Content>
</Popover.Root>
