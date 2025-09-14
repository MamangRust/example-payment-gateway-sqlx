import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useDeletePermanentAllSaldo from "@/hooks/admin/saldo/trashed/DeleteAll";

export function DeletePermanetAllSaldo() {
  const {
    handleSubmit,
    loadingDeletePermanentAllSaldoTrashed,
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  } = useDeletePermanentAllSaldo();

  return (
    <Dialog
      open={isModalVisibleDeletePermanentAll}
      onOpenChange={(open) =>
        open ? showModalDeletePermanentAll() : hideModalDeletePermanentAll()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Delete Saldo All</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to delete permanent all this Saldo?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalDeletePermanentAll}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingDeletePermanentAllSaldoTrashed ? "Deleting..." : "Delete"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
