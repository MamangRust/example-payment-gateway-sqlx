import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useDeleteTopup from "@/hooks/admin/topup/DeleteTopup";

export function DeleteTopup() {
  const {
    deleteTopupId,
    handleSubmit,
    loadingTrashedTopup,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  } = useDeleteTopup();

  return (
    <Dialog
      open={isModalVisibleDelete}
      onOpenChange={(open) =>
        open ? showModalDelete(deleteTopupId!) : hideModalDelete()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Delete Card</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to delete this card? This action cannot be
          undone.
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalDelete}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingTrashedTopup ? "Deleting..." : "Delete"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
