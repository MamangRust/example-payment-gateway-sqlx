import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useDeleteMerchant from "@/hooks/admin/merchant/DeleteMerchant";

export function DeleteMerchant() {
  const {
    deleteMerchantId,
    handleSubmit,
    loadingTrashedMerchant,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  } = useDeleteMerchant();

  return (
    <Dialog
      open={isModalVisibleDelete}
      onOpenChange={(open) =>
        open ? showModalDelete(deleteMerchantId!) : hideModalDelete()
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
            {loadingTrashedMerchant ? "Deleting..." : "Delete"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
