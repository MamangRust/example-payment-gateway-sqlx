import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useRoleStoreAllRole from "@/hooks/admin/role/trashed/RestoreAll";

export function RestoreAllRoleTrashed() {
  const {
    handleSubmit,
    loadingRestoreAllRoleTrashed,
    isModalVisibleRestoreAll,
    showModalRestoreAll,
    hideModalRestoreAll,
  } = useRoleStoreAllRole();

  return (
    <Dialog
      open={isModalVisibleRestoreAll}
      onOpenChange={(open) =>
        open ? showModalRestoreAll() : hideModalRestoreAll()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Restore All Transfer</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to restore all this transfer?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalRestoreAll}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingRestoreAllRoleTrashed ? "Restoring..." : "Restore"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
