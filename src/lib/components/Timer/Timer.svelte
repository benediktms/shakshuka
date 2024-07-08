<script lang="ts">
  import RadialProgress from '$lib/components/Timer/RadialProgress.svelte';
  import { Button } from '$ui/button';
  import { onMount } from 'svelte';
  import type { HTMLAttributes } from 'svelte/elements';

  let className: HTMLAttributes<HTMLDivElement>['class'] = undefined;
  export { className as class };

  export let timeFrame: number;

  type Interval = ReturnType<typeof setInterval>;

  let timePassed: number = 0;
  let interval: Interval | null = null;
  let initialTimestamp: number | null = null;
  let pauseTimestamp: number = 0;

  let isRunning = false;

  const startCount = (): void => {
    isRunning = true;
    if (interval) return;

    initialTimestamp = Date.now();

    interval = setInterval(() => {
      if (initialTimestamp !== null) {
        timePassed = Date.now() - initialTimestamp + pauseTimestamp;
      }
    }, 100);
  };

  const pauseCount = (): void => {
    isRunning = false;

    if (initialTimestamp !== null) {
      pauseTimestamp += Date.now() - initialTimestamp;
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
    initialTimestamp = null;
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
</script>

<div class={className}>
  <RadialProgress elapsed={Math.floor(timePassed / 1000)} totalMinutes={timeFrame} />
  {#if isRunning}
    <Button on:click={pauseCount} disabled={!isRunning}>stop timer</Button>
  {:else}
    <Button on:click={startCount} disabled={isRunning}>start timer</Button>
  {/if}
  <Button on:click={resetCount}>reset timer</Button>
</div>
