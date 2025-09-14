import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useRestoreTransfer from "@/hooks/admin/transfer/trashed/Restore";

export function RestoreTransfer() {
  const {
    restoreTransferId,
    handleSubmit,
    loadingRestoreTransferTrashed,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  } = useRestoreTransfer();

  return (
    <Dialog
      open={isModalVisibleRestore}
      onOpenChange={(open) =>
        open ? showModalRestore(restoreTransferId!) : hideModalRestore()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Restore Transfer</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to restore this transfer?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalRestore}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingRestoreTransferTrashed ? "Restore..." : "Restore"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
