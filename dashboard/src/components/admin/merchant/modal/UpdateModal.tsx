import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import UpdateMerchantForm from "../form/UpdateForm";
import useUpdateMerchant from "@/hooks/admin/merchant/UpdateMerchant";

export function UpdateMerchant() {
  const {
    merchant,
    editMerchantId,
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingUpdateMerchant,
    isModalVisibleEdit,
    showModalEdit,
    hideModalEdit,
  } = useUpdateMerchant();

  const defaultValues = merchant
    ? {
        user_id: {
          value: merchant.user_id?.toString(),
          label: "Loading...",
        },
        name: merchant.name,
        status: merchant.status,
      }
    : undefined;

  return (
    <Dialog
      open={isModalVisibleEdit}
      onOpenChange={(open) =>
        open ? showModalEdit(editMerchantId!) : hideModalEdit()
      }
    >
      <DialogContent className="max-w-2xl w-full">
        <DialogHeader>
          <DialogTitle>Update Merchant</DialogTitle>
        </DialogHeader>
        <UpdateMerchantForm
          onSubmit={handleSubmit}
          ref={formRef}
          defaultValues={defaultValues}
        />
        <DialogFooter>
          <Button variant="outline" onClick={hideModalEdit}>
            Cancel
          </Button>
          <Button variant="default" onClick={handleButtonSubmit}>
            {loadingUpdateMerchant ? "Updating..." : "Update"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
