<script lang="ts">
  import RadialProgress from '@/components/RadialProgress.svelte';
  import { Button } from '@/components/ui/button';
  import { onDestroy } from 'svelte';

  let timer = 0;
  let timeFrame = 1;
  let isRunning = false;

  let timerId: ReturnType<typeof setInterval> | null = null;

  export function startTimer() {
    if (timer === timeFrame * 60) {
      resetTimer();
    }

    timerId = setInterval(() => {
      if (timer < timeFrame * 60) {
        isRunning = true;
        timer += 1;
      } else {
        stopTimer();
        isRunning = false;
      }
    }, 1000);
  }

  export function stopTimer() {
    isRunning = false;
    if (timerId) {
      clearInterval(timerId);
    }
  }

  export function resetTimer() {
    timer = 0;
  }

  onDestroy(() => {
    stopTimer();
  });
</script>

<div class="container">
  <h1>Welcome to Shakshuka</h1>
  <RadialProgress elapsed={timer} totalMinutes={timeFrame} />
  {#if isRunning}
    <Button on:click={stopTimer} disabled={!isRunning}>stop timer</Button>
  {:else}
    <Button on:click={startTimer} disabled={isRunning}>start timer</Button>
  {/if}
  <Button on:click={resetTimer}>reset timer</Button>
</div>
