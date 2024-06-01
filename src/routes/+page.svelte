<script lang="ts">
  import Timer from '@/components/Timer/Timer.svelte';
  import { invoke } from '@tauri-apps/api';
  import { onMount } from 'svelte';
  import type { TestRes } from '../bindings/TestRes';

  let greeting: string;
  let testData: TestRes;
  async function getGreeting(name = 'Ben') {
    greeting = await invoke('greet', { name });
    testData = await invoke<TestRes>('test_command');
  }

  onMount(getGreeting);
</script>

<div class="container">
  <h1>Welcome to Shakshuka</h1>
  {greeting}
  {JSON.stringify(testData)}
  <Timer timeFrame={30} />
</div>
