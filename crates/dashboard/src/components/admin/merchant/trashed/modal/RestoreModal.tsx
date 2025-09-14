import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useRestoreMerchant from "@/hooks/admin/merchant/trashed/Restore";

export function RestoreMerchant() {
  const {
    restoreMerchantId,
    handleSubmit,
    loadingRestoreMerchantTrashed,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  } = useRestoreMerchant();

  return (
    <Dialog
      open={isModalVisibleRestore}
      onOpenChange={(open) =>
        open ? showModalRestore(restoreMerchantId!) : hideModalRestore()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Restore Merchant</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to restore this merchant?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalRestore}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingRestoreMerchantTrashed ? "Restore..." : "Restore"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
