import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useRestoreSaldo from "@/hooks/admin/saldo/trashed/Restore";

export function RestoreSaldo() {
  const {
    restoreSaldoId,
    handleSubmit,
    loadingRestoreSaldoTrashed,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  } = useRestoreSaldo();

  return (
    <Dialog
      open={isModalVisibleRestore}
      onOpenChange={(open) =>
        open ? showModalRestore(restoreSaldoId!) : hideModalRestore()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Restore Saldo</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to restore this Saldo?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalRestore}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingRestoreSaldoTrashed ? "Restore..." : "Restore"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
