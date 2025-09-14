import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useDeletePermanentMerchant from "@/hooks/admin/merchant/trashed/Delete";

export function DeletePermanetMerchant() {
  const {
    deletePermanentMerchantId,
    handleSubmit,
    loadingDeletePermanentMerchantTrashed,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  } = useDeletePermanentMerchant();

  return (
    <Dialog
      open={isModalVisibleDeletePermanent}
      onOpenChange={(open) =>
        open
          ? showModalDeletePermanent(deletePermanentMerchantId!)
          : hideModalDeletePermanent()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Delete Merchant</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to delete permanent this merchant?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalDeletePermanent}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingDeletePermanentMerchantTrashed ? "Deleting..." : "Delete"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
