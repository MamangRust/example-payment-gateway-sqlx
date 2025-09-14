import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useDeletePermanentAllWithdraw from "@/hooks/admin/withdraw/trashed/DeleteAll";

export function DeletePermanetAllWithdrawTrashed() {
  const {
    handleSubmit,
    loadingDeletePermanentAllWithdrawTrashed,
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  } = useDeletePermanentAllWithdraw();

  return (
    <Dialog
      open={isModalVisibleDeletePermanentAll}
      onOpenChange={(open) =>
        open ? showModalDeletePermanentAll() : hideModalDeletePermanentAll()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Delete Withdraw All</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to delete permanent all this withdraw?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalDeletePermanentAll}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingDeletePermanentAllWithdrawTrashed
              ? "Deleting..."
              : "Delete"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
