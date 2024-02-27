<script lang="ts">
  import { Button } from '@/components/ui/button';

  export let count = 0;

  export function setCount() {
    count += 1;
  }
</script>

<div class="container">
  <h1>Welcome to SvelteKit</h1>
  <p>count is {count}</p>
  <Button on:click={setCount}>increment count</Button>
</div>
