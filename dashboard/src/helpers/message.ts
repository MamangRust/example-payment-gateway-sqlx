import useNotificationStore from "@/store/notication";

const { addNotification } = useNotificationStore.getState();

export function handleMessageAction(entity: string, action: any) {
  let message = "";

  switch (action) {
    case "create":
      message = `${entity} has been successfully created.`;
      break;
    case "update":
      message = `${entity} details have been successfully updated.`;
      break;
    case "trashed":
      message = `${entity} has been moved to the trash.`;
      break;
    case "restore":
      message = `${entity} has been successfully restored.`;
      break;
    case "deletePermanent":
      message = `${entity} has been permanently deleted.`;
      break;
    case "restoreAll":
      message = `All ${entity}s have been successfully restored.`;
      break;
    case "deleteAllPermanent":
      message = `All ${entity}s have been permanently deleted.`;
      break;
    default:
      message = "Action not recognized.";
      break;
  }

  addNotification(message, action);
}
