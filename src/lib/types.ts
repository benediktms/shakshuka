export enum Status {
  Todo = 'Todo',
  InProgress = 'InProgress',
  Done = 'Done'
}

export type Task = {
  id: number;
  title: string;
  description: string;
  status: Status;
};

export type TaskColumn = { id: string; name: Status; items: Task[] };
