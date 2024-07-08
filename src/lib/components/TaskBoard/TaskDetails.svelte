<script lang="ts">
  import { Badge } from '$ui/badge';
  import { Button } from '$ui/button';
  import Card from '$ui/card';
  import { currentlyFocusedTaskId } from './taskDetailsStore';
  import type { Task } from './types';
  import { X } from 'lucide-svelte';

  export let task: Task;
</script>

<Card.Root class="mt-4 border-0 shadow-none">
  <div class="mx-5 flex items-end justify-end">
    <Button variant="ghost" size="icon" on:click={() => ($currentlyFocusedTaskId = undefined)}
      ><X /></Button
    >
  </div>
  <Card.Header>
    <span class="flex justify-between">
      <Card.Title>{task.title}</Card.Title>
      <Badge>{task.id}</Badge>
    </span>
  </Card.Header>
  <Card.Content>
    <div class="grid gap-6">
      <div class="grid gap-3 text-pretty">
        <p>{task.description}</p>
      </div>
    </div>
  </Card.Content>
</Card.Root>
