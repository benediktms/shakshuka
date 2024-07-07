<script lang="ts">
  import '../app.pcss';
  import { onDestroy, onMount } from 'svelte';
  import { theme } from '$lib/globalStores/themeStore';

  const changeHandler = (e: MediaQueryListEvent) => {
    $theme = e.matches ? 'dark' : 'light';
  };

  const mediaQueryList = window.matchMedia('(prefers-color-scheme: dark)');

  $theme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';

  $: document.documentElement.classList.toggle('dark', $theme === 'dark');

  onMount(() => {
    mediaQueryList.addEventListener('change', changeHandler);
  });
  onDestroy(() => mediaQueryList.removeEventListener('change', changeHandler));
</script>

<div class="h-dvh">
  <slot />
</div>
