import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useDeleteTransfer from "@/hooks/admin/transfer/DeleteTransfer";

export function DeleteTransfer() {
  const {
    deleteTransferId,
    handleSubmit,
    loadingTrashedTransfer,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  } = useDeleteTransfer();

  return (
    <Dialog
      open={isModalVisibleDelete}
      onOpenChange={(open) =>
        open ? showModalDelete(deleteTransferId!) : hideModalDelete()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Delete Card</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to delete this card?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalDelete}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingTrashedTransfer ? "Deleting..." : "Delete"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
