import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useDeletePermanentAllTransaction from "@/hooks/admin/transaction/trashed/DeleteAll";

export function DeletePermanetAllTransactionTrashed() {
  const {
    handleSubmit,
    loadingDeletePermanentAllTransactionTrashed,
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  } = useDeletePermanentAllTransaction();

  return (
    <Dialog
      open={isModalVisibleDeletePermanentAll}
      onOpenChange={(open) =>
        open ? showModalDeletePermanentAll() : hideModalDeletePermanentAll()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Delete Transaction All</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to delete permanent all this transaction?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalDeletePermanentAll}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingDeletePermanentAllTransactionTrashed
              ? "Deleting..."
              : "Delete"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
