import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import UpdateTransferForm from "../form/UpdateForm";
import useUpdateTransfer from "@/hooks/admin/transfer/UpdateTransfer";

export function UpdateTransfer() {
  const {
    editTransferId,
    transfer,
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingUpdateTransfer,
    isModalVisibleEdit,
    showModalEdit,
    hideModalEdit,
  } = useUpdateTransfer();

  const defaultValues = transfer
    ? {
        transfer_from: {
          value: transfer?.transfer_from,
          label: "Loading...",
        },
        transfer_to: {
          value: transfer?.transfer_to,
          label: "Loading...",
        },
        transfer_amount: Number(transfer.transfer_amount),
      }
    : undefined;

  return (
    <Dialog
      open={isModalVisibleEdit}
      onOpenChange={(open) =>
        open ? showModalEdit(editTransferId!) : hideModalEdit()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Edit Transfer</DialogTitle>
        </DialogHeader>
        <UpdateTransferForm
          onSubmit={handleSubmit}
          ref={formRef}
          defaultValues={defaultValues}
        />
        <DialogFooter>
          <Button variant="outline" onClick={hideModalEdit}>
            Cancel
          </Button>
          <Button variant="default" onClick={handleButtonSubmit}>
            {loadingUpdateTransfer ? "Updating..." : "Update"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
