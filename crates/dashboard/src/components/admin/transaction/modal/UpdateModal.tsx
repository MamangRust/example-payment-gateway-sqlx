import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import UpdateTransactionForm from "../form/UpdateForm";
import useUpdateTransaction from "@/hooks/admin/transaction/UpdateTransaction";

export function UpdateTransaction() {
  const {
    editTransactionId,
    transaction,
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingUpdateTransaction,
    isModalVisibleEdit,
    showModalEdit,
    hideModalEdit,
  } = useUpdateTransaction();

  const defaultValues = transaction
    ? {
        card_number: {
          value: transaction.card_number,
          label: "Loading...",
        },
        merchant_id: {
          value: transaction.merchant_id,
          label: "Loading...",
        },
        payment_method: transaction.payment_method,
        amount: transaction.amount,
        transaction_time: new Date(transaction.transaction_time),
      }
    : undefined;

  return (
    <Dialog
      open={isModalVisibleEdit}
      onOpenChange={(open) =>
        open ? showModalEdit(editTransactionId!) : hideModalEdit()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Edit New Transaction</DialogTitle>
        </DialogHeader>
        <UpdateTransactionForm
          onSubmit={handleSubmit}
          ref={formRef}
          defaultValues={defaultValues}
        />
        <DialogFooter>
          <Button variant="outline" onClick={hideModalEdit}>
            Cancel
          </Button>
          <Button variant="default" onClick={handleButtonSubmit}>
            {loadingUpdateTransaction ? "Updating..." : "Update"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
