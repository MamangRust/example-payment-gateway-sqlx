import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useDeletePermanentTransaction from "@/hooks/admin/transaction/trashed/Delete";

export function DeletePermanetTransactionTrashed() {
  const {
    deletePermanentTransactionId,
    handleSubmit,
    loadingDeletePermanentTransactionTrashed,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  } = useDeletePermanentTransaction();

  return (
    <Dialog
      open={isModalVisibleDeletePermanent}
      onOpenChange={(open) =>
        open
          ? showModalDeletePermanent(deletePermanentTransactionId!)
          : hideModalDeletePermanent()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Delete Transaction</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to delete permanent this transaction?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalDeletePermanent}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingDeletePermanentTransactionTrashed
              ? "Deleting..."
              : "Delete"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
