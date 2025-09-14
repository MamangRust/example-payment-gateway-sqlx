import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useRestoreAllTopup from "@/hooks/admin/topup/trashed/RestoreAll";

export function RestoreAllTopupTrashed() {
  const {
    handleSubmit,
    loadingRestoreAllTopupTrashed,
    isModalVisibleRestoreAll,
    showModalRestoreAll,
    hideModalRestoreAll,
  } = useRestoreAllTopup();

  return (
    <Dialog
      open={isModalVisibleRestoreAll}
      onOpenChange={(open) =>
        open ? showModalRestoreAll() : hideModalRestoreAll()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Restore All Topup</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to restore all this Topup?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalRestoreAll}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingRestoreAllTopupTrashed ? "Restoring..." : "Restore"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
