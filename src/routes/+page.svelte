<script lang="ts">
  import { Button } from '@/components/ui/button';
  import RadialProgress from '@/components/RadialProgress.svelte';
  import { onDestroy, onMount } from 'svelte';

  export let timeFrame: number;

  type Interval = ReturnType<typeof setInterval>;

  let timePassed: number = 0;
  let interval: Interval | null = null;
  let initalTimestamp: number | null = null;
  let pauseTimestamp: number = 0;

  let isRunning = false;

  const startCount = (): void => {
    isRunning = true;
    if (interval) return;

    initalTimestamp = Date.now();

    interval = setInterval(() => {
      if (initalTimestamp !== null) {
        timePassed = Date.now() - initalTimestamp + pauseTimestamp;
      }
    }, 100);
  };

  const pauseCount = (): void => {
    isRunning = false;

    if (initalTimestamp !== null) {
      pauseTimestamp += Date.now() - initalTimestamp;
    }

    if (interval) {
      clearInterval(interval);
      interval = null;
    }
  };

  const resetCount = (): void => {
    isRunning = false;
    if (interval) {
      clearInterval(interval);
    }
    initalTimestamp = null;
    pauseTimestamp = 0;
    interval = null;
    timePassed = 0;
  };

  onMount(() => {
    return () => {
      if (interval !== null) {
        clearInterval(interval);
      }
    };
  });

  onDestroy(() => {
    if (interval) {
      clearInterval(interval);
    }
  });
</script>

<div class="container">
  <h1>Welcome to Shakshuka</h1>
  <RadialProgress elapsed={Math.floor(timePassed / 1000)} totalMinutes={timeFrame} />
  {#if isRunning}
    <Button on:click={pauseCount} disabled={!isRunning}>stop timer</Button>
  {:else}
    <Button on:click={startCount} disabled={isRunning}>start timer</Button>
  {/if}
  <Button on:click={resetCount}>reset timer</Button>
</div>
