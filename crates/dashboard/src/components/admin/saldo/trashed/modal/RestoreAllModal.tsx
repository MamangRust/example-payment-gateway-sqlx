import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useRestoreAllSaldo from "@/hooks/admin/saldo/trashed/RestoreAll";

export function RestoreAllSaldo() {
  const {
    handleSubmit,
    loadingRestoreAllSaldoTrashed,
    isModalVisibleRestoreAll,
    showModalRestoreAll,
    hideModalRestoreAll,
  } = useRestoreAllSaldo();

  return (
    <Dialog
      open={isModalVisibleRestoreAll}
      onOpenChange={(open) =>
        open ? showModalRestoreAll() : hideModalRestoreAll()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Restore All Saldo</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to restore all this Saldo?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalRestoreAll}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingRestoreAllSaldoTrashed ? "Restoring..." : "Restore"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
