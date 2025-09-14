import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useUpdateSaldo from "@/hooks/admin/saldo/UpdateSaldo";
import UpdateSaldoForm from "../form/UpdateForm";

export function UpdateSaldo() {
  const {
    editSaldoId,
    formRef,
    saldo,
    handleButtonSubmit,
    handleSubmit,
    loadingUpdateSaldo,
    isModalVisibleEdit,
    showModalEdit,
    hideModalEdit,
  } = useUpdateSaldo();

  const defaultValues = saldo
    ? {
        card_number: {
          value: saldo.card_number,
          label: saldo.card_number,
        },
        total_balance: saldo.total_balance,
      }
    : undefined;

  return (
    <Dialog
      open={isModalVisibleEdit}
      onOpenChange={(open) =>
        open ? showModalEdit(editSaldoId!) : hideModalEdit()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Edit New Saldo</DialogTitle>
        </DialogHeader>
        <UpdateSaldoForm
          onSubmit={handleSubmit}
          ref={formRef}
          defaultValues={defaultValues}
        />
        <DialogFooter>
          <Button variant="outline" onClick={hideModalEdit}>
            Cancel
          </Button>
          <Button variant="default" onClick={handleButtonSubmit}>
            {loadingUpdateSaldo ? "Updating..." : "Update"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
