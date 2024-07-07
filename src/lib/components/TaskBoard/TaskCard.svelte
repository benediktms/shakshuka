<script lang="ts">
  import Card from '$ui/card';
  import Collapsible from '$ui/collapsible';
  import { ChevronsDown, ChevronsUp } from 'lucide-svelte';
  import type { HTMLAttributes } from 'svelte/elements';
  import { Badge } from '$ui/badge';
  import type { Task } from '$lib/components/TaskBoard/types';
  import { onDestroy } from 'svelte';
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

  let hoverTimer: ReturnType<typeof setTimeout>;

  const debounce = (value: boolean) => {
    clearTimeout(hoverTimer);
    hoverTimer = setTimeout(
      () => {
        if (canBeExpanded && !isLocked) {
          isExpanded = value;
        }
      },
      value ? 200 : 100
    );
  };

  function handleExpansion() {
    if (isExpanded && !isLocked) {
      isLocked = true;
    } else if (!isExpanded && !isLocked) {
      isExpanded = true;
      isLocked = true;
    } else if (isExpanded && isLocked) {
      isExpanded = false;
      isLocked = false;
    }
  }

  onDestroy(() => clearTimeout(hoverTimer));
</script>

<Card.Root class={className}>
  <Collapsible.Root bind:open={isExpanded}>
    <Collapsible.Trigger class="hidden" />
    <div
      role="contentinfo"
      on:mouseenter={() => debounce(true)}
      on:mouseleave={() => debounce(false)}
    >
      <Card.Header>
        <div class="align-center flex items-center justify-between">
          <Card.Title>{title}</Card.Title>
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
            <Badge variant="outline">{id}</Badge>
          </span>
        </div>
        <a href="#noopener" on:click={() => ($currentlyFocusedTaskId = id)}>
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
    </div>
  </Collapsible.Root>
</Card.Root>
