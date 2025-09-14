import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useRestoreCard from "@/hooks/admin/card/trashed/Restore";
export function RestoreCard() {
  const {
    restoreCardId,
    handleSubmit,
    loadingRestoreCardTrashed,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  } = useRestoreCard();

  return (
    <Dialog
      open={isModalVisibleRestore}
      onOpenChange={(open) =>
        open ? showModalRestore(restoreCardId!) : hideModalRestore()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Restore Card</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to restore this card?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalRestore}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingRestoreCardTrashed ? "Restore..." : "Restore"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
