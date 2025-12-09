import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useDeletePermanentCard from "@/hooks/admin/card/trashed/Delete";

export function DeletePermanetCard() {
  const {
    handleSubmit,
    deletePermanentCardId,
    loadingDeletePermanentCardTrashed,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  } = useDeletePermanentCard();

  return (
    <Dialog
      open={isModalVisibleDeletePermanent}
      onOpenChange={(open) =>
        open
          ? showModalDeletePermanent(deletePermanentCardId!)
          : hideModalDeletePermanent()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Delete Card</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to delete permanent this card?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalDeletePermanent}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingDeletePermanentCardTrashed ? "Deleting..." : "Delete"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
