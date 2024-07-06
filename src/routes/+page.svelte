<script lang="ts">
  import TaskCard from '$lib/components/TaskCard.svelte';

  enum Status {
    Todo,
    InProgress,
    Done
  }

  type Task = { title: string; description: string; content?: string; status: Status };
  const tasks: Array<Task> = [
    { title: 'Task', description: 'Task desciption', status: Status.Todo },
    { title: 'Task', description: 'Task desciption', status: Status.Todo },
    { title: 'Task', description: 'Task desciption', status: Status.InProgress },
    { title: 'Task', description: 'Task desciption', status: Status.InProgress },
    { title: 'Task', description: 'Task desciption', status: Status.Done },
    { title: 'Task', description: 'Task desciption', status: Status.Done }
  ];

  const tasksTodo = tasks.filter(t => t.status === Status.Todo);
  const tasksInProgress = tasks.filter(t => t.status === Status.InProgress);
  const tasksDone = tasks.filter(t => t.status === Status.Done);
</script>

<div class="container h-full">
  <div class="grid-gap-5 grid h-full grid-cols-3 p-3">
    <div class="flex flex-col px-2">
      <h2 class="my-4 text-center text-lg">To do</h2>
      {#each tasksTodo as { title, description }}
        <TaskCard class="mb-2" {title} {description} />
      {/each}
    </div>
    <div class="flex flex-col px-2">
      <h2 class="my-4 text-center text-lg">In Progress</h2>
      {#each tasksInProgress as { title, description }}
        <TaskCard class="mb-2" {title} {description} />
      {/each}
    </div>
    <div class="flex flex-col px-2">
      <h2 class="my-4 text-center text-lg">Done</h2>
      {#each tasksDone as { title, description }}
        <TaskCard class="mb-2" {title} {description} />
      {/each}
    </div>
  </div>
</div>
