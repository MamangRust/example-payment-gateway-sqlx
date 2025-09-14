import { Notification } from "../model";

export interface NotificationState {
  notifications: Notification[];
  addNotification: (message: string) => void;
  removeNotification: (id: number) => void;
}
