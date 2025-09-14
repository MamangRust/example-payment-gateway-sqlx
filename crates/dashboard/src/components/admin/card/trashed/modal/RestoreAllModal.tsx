import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useRestoreAllCard from "@/hooks/admin/card/trashed/RestoreAll";

export function RestoreAllCard() {
  const {
    handleSubmit,
    loadingRestoreAllCardTrashed,
    isModalVisibleRestoreAll,
    showModalRestoreAll,
    hideModalRestoreAll,
  } = useRestoreAllCard();

  return (
    <Dialog
      open={isModalVisibleRestoreAll}
      onOpenChange={(open) =>
        open ? showModalRestoreAll() : hideModalRestoreAll()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Restore All Card</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to restore all this card?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalRestoreAll}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingRestoreAllCardTrashed ? "Restoring..." : "Restore"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
