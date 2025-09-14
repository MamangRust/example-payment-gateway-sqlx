import React, { useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
  DropdownMenuItem,
  DropdownMenuSeparator,
} from "@/components/ui/dropdown-menu";
import {
  Bell,
  Plus,
  Edit,
  Trash2,
  Undo,
  Delete,
  RotateCcw,
  RotateCw,
  MessageSquare,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { NotificationCommand } from "./command-notification";
import { Notification } from "@/types/model";
import useNotificationStore from "@/store/notication";

const getIcon = (type: Notification["type"]) => {
  switch (type) {
    case "create":
      return <Plus className="mr-2 h-4 w-4 text-green-500" />;
    case "update":
      return <Edit className="mr-2 h-4 w-4 text-blue-500" />;
    case "trashed":
      return <Trash2 className="mr-2 h-4 w-4 text-yellow-600" />;
    case "restore":
      return <Undo className="mr-2 h-4 w-4 text-emerald-600" />;
    case "delete":
      return <Delete className="mr-2 h-4 w-4 text-red-600" />;
    case "restoreAll":
      return <RotateCcw className="mr-2 h-4 w-4 text-teal-600" />;
    case "deleteAll":
      return <RotateCw className="mr-2 h-4 w-4 text-rose-600" />;
    default:
      return <MessageSquare className="mr-2 h-4 w-4 text-gray-500" />;
  }
};

export function NotificationMenu() {
  const [openCommand, setOpenCommand] = useState(false);
  const { notifications, removeNotification } = useNotificationStore();
  const displayedNotifications = notifications.slice(0, 3);

  React.useEffect(() => {
    const handleShortcut = (e: KeyboardEvent) => {
      if (e.key === "d" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        setOpenCommand(true);
      }
    };

    document.addEventListener("keydown", handleShortcut);
    return () => document.removeEventListener("keydown", handleShortcut);
  }, []);

  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="ghost" size="icon" className="relative">
            <Bell className="h-5 w-5" />
            {notifications.length > 0 && (
              <span className="absolute right-1 top-1 inline-flex h-2 w-2 rounded-full bg-red-500"></span>
            )}
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end" className="w-64">
          {notifications.length === 0 ? (
            <DropdownMenuItem disabled className="text-gray-500">
              No notifications
            </DropdownMenuItem>
          ) : (
            <>
              {displayedNotifications.map((notification: any) => (
                <DropdownMenuItem
                  key={notification.id}
                  onSelect={() => removeNotification(notification.id)}
                >
                  <div className="flex items-center">
                    {getIcon(notification.type)}
                    <span>{notification.message}</span>
                  </div>
                </DropdownMenuItem>
              ))}
              {notifications.length > 3 && (
                <>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem
                    className="text-blue-600"
                    onClick={() => setOpenCommand(true)}
                  >
                    <span>See all notifications</span>
                  </DropdownMenuItem>
                </>
              )}
            </>
          )}
        </DropdownMenuContent>
      </DropdownMenu>

      <NotificationCommand open={openCommand} setOpen={setOpenCommand} />
    </>
  );
}
