export enum Status {
  TODO = 'TODO',
  IN_PROGRESS = 'IN_PROGRESS',
  DONE = 'DONE'
}

export type Task = {
  id: string;
  title: string;
  description: string;
  status: Status;
};

export type TaskColumnId = Lowercase<
  `task-${Status.TODO}-column` | `task-${Status.DONE}-column` | `task-${Status.IN_PROGRESS}-column`
>;

export type TaskColumn = { id: TaskColumnId; name: Status; items: Task[] };
export type ColumnType = 'targetable' | 'untargetable';
