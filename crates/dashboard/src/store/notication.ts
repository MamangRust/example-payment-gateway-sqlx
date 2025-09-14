import { Notification } from "@/types/model";
import { create } from "zustand";
import { persist } from "zustand/middleware";

type NotificationState = {
  notifications: Notification[];
  addNotification: (message: string, type?: Notification["type"]) => void;
  removeNotification: (id: number) => void;
  clearAllNotifications: () => void;
};

const useNotificationStore = create<NotificationState>()(
  persist(
    (set) => ({
      notifications: [],
      addNotification: (message, type) => {
        set((state) => ({
          notifications: [
            ...state.notifications,
            { id: Date.now(), message, type },
          ],
        }));
      },
      removeNotification: (id) => {
        set((state) => ({
          notifications: state.notifications.filter(
            (notification) => notification.id !== id,
          ),
        }));
      },
      clearAllNotifications: () => {
        set({ notifications: [] });
      },
    }),
    {
      name: "notification-storage",
    },
  ),
);

export default useNotificationStore;
