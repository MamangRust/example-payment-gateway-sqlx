import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import UpdateWithdrawForm from "../form/UpdateForm";
import useUpdateWithdraw from "@/hooks/admin/withdraw/UpdateWithdraw";

export function UpdateWithdraw() {
  const {
    formRef,
    withdraw,
    handleButtonSubmit,
    editWithdrawId,
    handleSubmit,
    loadingUpdateWithdraw,
    isModalVisibleEdit,
    showModalEdit,
    hideModalEdit,
  } = useUpdateWithdraw();

  const defaultValues = withdraw
    ? {
        card_number: {
          value: withdraw.card_number,
          label: "Loading...",
        },
        withdraw_amount: withdraw.withdraw_amount,
        withdraw_time: withdraw.withdraw_time,
      }
    : undefined;

  console.log("defaultValues", defaultValues);

  return (
    <Dialog
      open={isModalVisibleEdit}
      onOpenChange={(open) =>
        open ? showModalEdit(editWithdrawId!) : hideModalEdit()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Edit Withdraw</DialogTitle>
        </DialogHeader>
        <UpdateWithdrawForm
          onSubmit={handleSubmit}
          ref={formRef}
          defaultValues={defaultValues}
        />
        <DialogFooter>
          <Button variant="outline" onClick={hideModalEdit}>
            Cancel
          </Button>
          <Button variant="default" onClick={handleButtonSubmit}>
            {loadingUpdateWithdraw ? "Updating..." : "Update"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
