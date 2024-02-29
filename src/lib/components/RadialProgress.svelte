<script lang="ts">
  export let elapsed: number;

  const full = 25 * 60;
  const radius = 40;
  const circumference = 2 * Math.PI * radius;

  $: dashOffset = circumference * (1 - elapsed / full);
</script>

<div class="relative h-40 w-40">
  <svg class="h-full w-full" viewBox="0 0 100 100">
    <circle
      class="stroke-current text-gray-200 dark:text-gray-700"
      stroke-width="10"
      cx="50"
      cy="50"
      r={radius}
      fill="transparent"
    ></circle>
    <circle
      class="progress-ring stroke-current text-indigo-500 dark:text-indigo-400"
      stroke-width="10"
      stroke-linecap="round"
      cx="50"
      cy="50"
      r={radius}
      fill="transparent"
      stroke-dasharray={`${circumference} ${circumference}`}
      stroke-dashoffset={dashOffset}
    ></circle>
    <text
      class="dark:fill-white"
      x="50"
      y="50"
      font-size="12"
      text-anchor="middle"
      alignment-baseline="middle"
    >
      70%
    </text>
  </svg>
</div>

<style>
  .progress-ring {
    transition: stroke-dashoffset 0.35s;
    transform: rotate(-90deg);
    transform-origin: 50% 50%;
  }
</style>
