<script lang="ts">
  import '../app.pcss';
  import { onDestroy, onMount } from 'svelte';
  import { theme } from '$lib/globalStores/themeStore';
  import CommandPallete from '$lib/components/CommandPallete/CommandPallete.svelte';

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
  <div
    data-tauri-drag-region
    class="fixed left-0 right-0 top-0 flex h-[34px] w-full justify-around bg-decoration"
  >
    <div class="pointer-events-none flex h-full items-center">
      <div class="pointer-events-auto">
        <CommandPallete />
      </div>
    </div>
  </div>
  <slot />
</div>
