<script lang="ts">
  import { Button } from '$lib/components/ui/button';
  import Card from '$ui/card';
  import Collapsible from '$ui/collapsible';
  import ChevronsUp from 'lucide-svelte/icons/chevrons-up';
  import ChevronsDown from 'lucide-svelte/icons/chevrons-down';
  import type { HTMLAttributes } from 'svelte/elements';
  import { Badge } from '$ui/badge';
  import type { Task } from '$lib/types';

  let className: HTMLAttributes<HTMLDivElement>['class'] = undefined;
  export { className as class };

  export let task: Task;
  const { title, description, id } = task;

  let isExpanded = false;
  const truncatedDescription = description.slice(0, 80).concat('...');
  const isExpandable = truncatedDescription.length < description.length;
</script>

<Card.Root class={className}>
  <Collapsible.Root>
    <Card.Header>
      <div class="flex items-center justify-between">
        <Card.Title>{title}</Card.Title>
        <Badge variant="outline">{id}</Badge>
      </div>
      {#if !isExpanded}
        <Card.Description>
          {isExpandable ? truncatedDescription : description}
        </Card.Description>
      {/if}
      <Collapsible.Content>
        <Card.Description>
          {description}
        </Card.Description>
      </Collapsible.Content>
    </Card.Header>
    {#if isExpandable}
      <div class="ml-5 flex items-center justify-between p-1">
        <h4 class="text-xs font-semibold">{isExpanded ? 'less' : 'details'}</h4>
        <Collapsible.Trigger asChild let:builder>
          <Button
            builders={[builder]}
            variant="ghost"
            size="sm"
            class="w-9 p-2"
            on:click={() => (isExpanded = !isExpanded)}
          >
            {#if !isExpanded}
              <ChevronsDown class="h-4 w-4" />
            {:else}
              <ChevronsUp class="h-4 w-4" />
            {/if}
            <span class="sr-only">Toggle</span>
          </Button>
        </Collapsible.Trigger>
      </div>
    {/if}
  </Collapsible.Root>
</Card.Root>
