import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useDeletePermanentAllMerchant from "@/hooks/admin/merchant/trashed/DeleteAll";

export function DeletePermanetAllMerchant() {
  const {
    handleSubmit,
    loadingDeletePermanentAllMerchantTrashed,
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  } = useDeletePermanentAllMerchant();

  return (
    <Dialog
      open={isModalVisibleDeletePermanentAll}
      onOpenChange={(open) =>
        open ? showModalDeletePermanentAll() : hideModalDeletePermanentAll()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Delete Merchant All</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to delete permanent all this merchant?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalDeletePermanentAll}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingDeletePermanentAllMerchantTrashed
              ? "Deleting..."
              : "Delete"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
