<script lang="ts">
  import { Sun } from 'lucide-svelte';
  import { Moon } from 'lucide-svelte';
  import { Button } from '$lib/components/ui/button';
  import { onDestroy, onMount } from 'svelte';

  let theme = document.documentElement.classList.contains('dark') ? 'dark' : 'light';

  const toggleTheme = () => {
    theme = theme === 'light' ? 'dark' : 'light';
    document.documentElement.classList.toggle('dark', theme === 'dark');
  };

  const handleSystemThemeChange = (_: unknown) => {
    if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      theme = 'dark';
    } else {
      theme = 'light';
    }
    document.documentElement.classList.toggle('dark', theme === 'dark');
  };

  onMount(() => {
    window
      .matchMedia('(prefers-color-scheme: dark)')
      .addEventListener('change', handleSystemThemeChange);
  });

  onDestroy(() => {
    window
      .matchMedia('(prefers-color-scheme: dark)')
      .removeEventListener('change', handleSystemThemeChange);
  });
</script>

<Button on:click={toggleTheme} variant="outline" size="icon">
  <Sun
    class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
  />
  <Moon
    class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
  />
  <span class="sr-only">Toggle theme</span>
</Button>
