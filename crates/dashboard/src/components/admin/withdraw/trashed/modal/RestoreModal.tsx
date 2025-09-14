import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useRestoreWithdraw from "@/hooks/admin/withdraw/trashed/Restore";

export function RestoreWithdraw() {
  const {
    restoreWithdrawId,
    handleSubmit,
    loadingRestoreWithdrawTrashed,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  } = useRestoreWithdraw();

  return (
    <Dialog
      open={isModalVisibleRestore}
      onOpenChange={(open) =>
        open ? showModalRestore(restoreWithdrawId!) : hideModalRestore()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Restore Withdraw</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to restore this withdraw?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalRestore}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingRestoreWithdrawTrashed ? "Restore..." : "Restore"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
