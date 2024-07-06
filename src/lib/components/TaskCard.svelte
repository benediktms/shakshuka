<script lang="ts">
  import { Button } from '$lib/components/ui/button';
  import Card from '$ui/card';
  import Collapsible from '$ui/collapsible';
  import ChevronsUp from 'lucide-svelte/icons/chevrons-up';
  import ChevronsDown from 'lucide-svelte/icons/chevrons-down';
  import type { HTMLAttributes } from 'svelte/elements';
  import Separator from './ui/separator/separator.svelte';
  import { fade } from 'svelte/transition';

  let className: HTMLAttributes<HTMLDivElement>['class'] = undefined;
  export { className as class };

  export let title: string;
  export let description: string;

  let isExpanded = false;

  const truncatedDescription = description.slice(0, 80).concat('...');
</script>

<Card.Root class={className}>
  <Card.Header>
    <Card.Title>{title}</Card.Title>
    {#if !isExpanded && truncatedDescription.length < description.length}
      <Card.Description>{truncatedDescription}</Card.Description>
    {:else}
      <Card.Description>{description}</Card.Description>
    {/if}
  </Card.Header>
  {#if $$slots.content || $$slots.footer}
    <Collapsible.Root class="space-y-2">
      <div class="ml-5 flex items-center justify-between space-x-4 p-1">
        {#if !isExpanded}
          <h4 class="text-xs font-semibold">details</h4>
        {:else}
          <span class="w-full">
            <Separator class="w-full" />
          </span>
        {/if}
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
      <Collapsible.Content class="space-y-2">
        <div transition:fade>
          {#if $$slots.content}
            <Card.Content>
              <slot name="content" />
            </Card.Content>
          {/if}
          {#if $$slots.footer}
            <Card.Footer>
              <slot name="footer" />
            </Card.Footer>
          {/if}
        </div>
      </Collapsible.Content>
    </Collapsible.Root>
  {/if}
</Card.Root>
