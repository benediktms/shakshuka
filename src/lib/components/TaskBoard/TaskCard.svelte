<script lang="ts">
  import Card from '$ui/card';
  import Collapsible from '$ui/collapsible';
  import { ChevronsDown, ChevronsUp } from 'lucide-svelte';
  import type { HTMLAttributes } from 'svelte/elements';
  import { Badge } from '$ui/badge';
  import type { Task } from '$lib/components/TaskBoard/types';
  import Button from '$ui/button/button.svelte';
  import { currentlyFocusedTaskId } from './taskDetailsStore';

  let className: HTMLAttributes<HTMLDivElement>['class'] = undefined;
  export { className as class };

  export let task: Task;
  const { title, description, id } = task;

  let isExpanded = false;
  let isLocked = false;
  const truncatedDescription = description.slice(0, 80).concat('...');

  const canBeExpanded = truncatedDescription.length < description.length;

  function handleExpansion() {
    if (isExpanded && !isLocked) {
      isLocked = true;
    } else if (isExpanded && isLocked) {
      isExpanded = false;
      isLocked = false;
    } else if (!isExpanded && !isLocked) {
      isExpanded = true;
      isLocked = true;
    }
  }
</script>

<Card.Root class={className}>
  <Collapsible.Root bind:open={isExpanded}>
    <Collapsible.Trigger class="hidden" />
    <Card.Header>
      <div class="align-center flex items-center justify-between">
        <Card.Title class="text-nowrap">{title}</Card.Title>
        <span class="align-center flex items-center justify-evenly">
          {#if canBeExpanded}
            <Button size="icon" variant="ghost" on:click={handleExpansion}>
              {#if isLocked}
                <ChevronsUp class="h-4 w-4" />
              {:else}
                <ChevronsDown class="h-4 w-4" />
              {/if}
            </Button>
          {/if}
          <Badge
            class="text-nowrap"
            variant={$currentlyFocusedTaskId === id ? 'default' : 'outline'}>{id}</Badge
          >
        </span>
      </div>
      <a
        href="#noopener"
        on:click={() => {
          if ($currentlyFocusedTaskId === id) {
            $currentlyFocusedTaskId = undefined;
            isExpanded = false;
            isLocked = false;
          } else {
            $currentlyFocusedTaskId = id;
            isExpanded = true;
            isLocked = true;
          }
        }}
      >
        {#if !isExpanded}
          <Card.Description>
            {canBeExpanded ? truncatedDescription : description}
          </Card.Description>
        {/if}
        <Collapsible.Content>
          <Card.Description>
            {description}
          </Card.Description>
        </Collapsible.Content>
      </a>
    </Card.Header>
  </Collapsible.Root>
</Card.Root>
