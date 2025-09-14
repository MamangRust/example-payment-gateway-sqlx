import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useDeleteSaldo from "@/hooks/admin/saldo/DeleteSaldo";

export function DeleteSaldo() {
  const {
    deleteSaldoId,
    handleSubmit,
    loadingTrashedSaldo,
    isModalVisibleDelete,
    showModalDelete,
    hideModalDelete,
  } = useDeleteSaldo();

  return (
    <Dialog
      open={isModalVisibleDelete}
      onOpenChange={(open) =>
        open ? showModalDelete(deleteSaldoId!) : hideModalDelete()
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
            {loadingTrashedSaldo ? "Deleting..." : "Delete"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
