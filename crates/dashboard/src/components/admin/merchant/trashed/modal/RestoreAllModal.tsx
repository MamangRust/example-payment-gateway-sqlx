import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useRestoreAllMerchant from "@/hooks/admin/merchant/trashed/RestoreAll";

export function RestoreAllMerchant() {
  const {
    handleSubmit,
    loadingRestoreAllMerchantTrashed,
    isModalVisibleRestoreAll,
    showModalRestoreAll,
    hideModalRestoreAll,
  } = useRestoreAllMerchant();

  return (
    <Dialog
      open={isModalVisibleRestoreAll}
      onOpenChange={(open) =>
        open ? showModalRestoreAll() : hideModalRestoreAll()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Restore All Merchant</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to restore all this merchant?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalRestoreAll}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingRestoreAllMerchantTrashed ? "Restoring..." : "Restore"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
