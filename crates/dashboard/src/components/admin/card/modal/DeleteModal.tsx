import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useDeleteCard from "@/hooks/admin/card/DeleteCard";

export function DeleteCard() {
  const {
    deleteCardId,
    handleSubmit,
    loadingTrashedCard,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  } = useDeleteCard();

  return (
    <Dialog
      open={isModalVisibleDelete}
      onOpenChange={(open) =>
        open ? showModalDelete(deleteCardId!) : hideModalDelete()
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
            {loadingTrashedCard ? "Deleting..." : "Delete"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
