import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import UpdateTopupForm from "../form/UpdateForm";
import useUpdateTopup from "@/hooks/admin/topup/UpdateTopup";

export function UpdateTopup() {
  const {
    editTopupId,
    topup,
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingUpdateTopup,
    isModalVisibleEdit,
    showModalEdit,
    hideModalEdit,
  } = useUpdateTopup();

  const defaultValues = topup
    ? {
        card_number: {
          value: topup.card_number,
          label: "Loading...",
        },
        topup_amount: Number(topup.topup_amount),
        topup_method: topup.topup_method,
      }
    : undefined;

  return (
    <Dialog
      open={isModalVisibleEdit}
      onOpenChange={(open) =>
        open ? showModalEdit(editTopupId!) : hideModalEdit()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Edit New Top-up</DialogTitle>
        </DialogHeader>
        <UpdateTopupForm
          onSubmit={handleSubmit}
          ref={formRef}
          defaultValues={defaultValues}
        />
        <DialogFooter>
          <Button variant="outline" onClick={hideModalEdit}>
            Cancel
          </Button>
          <Button variant="default" onClick={handleButtonSubmit}>
            {loadingUpdateTopup ? "Updating..." : "Update"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
