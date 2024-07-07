<script lang="ts">
  import Board from '$lib/components/TaskBoard/Board.svelte';
  import { cn } from '$lib/utils';
  import type { HTMLAttributes } from 'svelte/elements';

  let className: HTMLAttributes<HTMLDivElement>['class'] = undefined;
  export { className as class };
</script>

<Board class={cn(className, 'h-full pt-[38px]')} />
