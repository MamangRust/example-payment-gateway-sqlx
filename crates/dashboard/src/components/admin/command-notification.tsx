import React from "react";
import { MessageSquare } from "lucide-react";
import {
  CommandItem,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandList,
} from "@/components/ui/command";
import { Notification } from "@/types/model";
import useNotificationStore from "@/store/notication";

type NotificationCommandProps = {
  open: boolean;
  setOpen: (open: boolean) => void;
};

export function NotificationCommand({
  open,
  setOpen,
}: NotificationCommandProps) {
  const { notifications, removeNotification } = useNotificationStore();

  React.useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === "d" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        setOpen(!open);
      }
    };
    document.addEventListener("keydown", down);
    return () => document.removeEventListener("keydown", down);
  }, [setOpen]);

  return (
    <>
      <CommandDialog open={open} onOpenChange={setOpen}>
        <CommandInput placeholder="Type a command or search..." />
        <CommandList>
          <CommandEmpty>No results found.</CommandEmpty>
          <CommandGroup heading="Notifications">
            {notifications.map((notification: Notification) => (
              <CommandItem
                key={notification.id}
                onSelect={() => removeNotification(notification.id)}
              >
                <MessageSquare className="mr-2 h-4 w-4 text-blue-500" />
                <span>{notification.message}</span>
              </CommandItem>
            ))}
          </CommandGroup>
        </CommandList>
      </CommandDialog>
    </>
  );
}
