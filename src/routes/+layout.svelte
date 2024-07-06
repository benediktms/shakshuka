<script lang="ts">
  import '../app.pcss';
  import { onMount } from 'svelte';
  import { theme } from '$lib/stores/themeStore';

  onMount(() => {
    const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches
      ? 'dark'
      : 'light';
    theme.set(systemTheme);

    const mediaQueryList = window.matchMedia('(prefers-color-scheme: dark)');
    const changeHandler = (e: MediaQueryListEvent) => {
      theme.set(e.matches ? 'dark' : 'light');
    };
    mediaQueryList.addEventListener('change', changeHandler);

    const unsubscribe = theme.subscribe(value => {
      document.documentElement.classList[value === 'dark' ? 'add' : 'remove']('dark');
    });

    return () => {
      unsubscribe();
      mediaQueryList.removeEventListener('change', changeHandler);
    };
  });
</script>

<div class="h-dvh">
  <slot />
</div>
