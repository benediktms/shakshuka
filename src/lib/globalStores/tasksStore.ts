import { Status, type Task, type TaskColumn } from '$lib/components/TaskBoard/types';
import { derived, writable } from 'svelte/store';

const dummyTasks = Array.from(new Array(24)).map<Task>((e, i) => ({
  id: `TASK-${i}`,
  title: `Task ${i}`,
  description:
    i % 3 === 0
      ? 'Lorem ipsum dolor sit amet consectetur adipisicing elit. Ipsam dignissimos vitae quos excepturi laboriosam ut consequatur sed dolorem amet reiciendis eos quas voluptas voluptatem dicta, minima, corporis soluta libero sunt!'
      : 'Lorem ipsum dolor sit amet consectetur adipisicing elit.',
  status: i < 8 ? Status.TODO : i < 16 && i >= 8 ? Status.IN_PROGRESS : Status.DONE
}));

export const tasksColumns = writable<TaskColumn[]>([
  {
    id: 'task-todo-column',
    name: Status.TODO,
    items: dummyTasks.filter(t => t.status === Status.TODO)
  },
  {
    id: 'task-in_progress-column',
    name: Status.IN_PROGRESS,
    items: dummyTasks.filter(t => t.status === Status.IN_PROGRESS)
  },
  {
    id: 'task-done-column',
    name: Status.DONE,
    items: dummyTasks.filter(t => t.status === Status.DONE)
  }
]);

export const allTasks = derived(tasksColumns, columns => columns.flatMap(c => c.items));
