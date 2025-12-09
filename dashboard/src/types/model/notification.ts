export interface Notification {
  id: number;
  message: string;
  type?:
    | "create"
    | "update"
    | "trashed"
    | "restore"
    | "delete"
    | "restoreAll"
    | "deleteAll";
}
