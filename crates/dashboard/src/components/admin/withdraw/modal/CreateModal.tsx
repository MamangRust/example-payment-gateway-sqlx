import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import CreateWithdrawForm from "../form/CreateForm";
import useCreateWithdraw from "@/hooks/admin/withdraw/CreateWithdraw";

export function AddWithdraw() {
  const {
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingCreateWithdraw,
    isModalVisible,
    showModal,
    hideModal,
  } = useCreateWithdraw();

  return (
    <Dialog
      open={isModalVisible}
      onOpenChange={(open) => (open ? showModal() : hideModal())}
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Add New Withdraw</DialogTitle>
        </DialogHeader>
        <CreateWithdrawForm onSubmit={handleSubmit} ref={formRef} />
        <DialogFooter>
          <Button variant="outline" onClick={hideModal}>
            Cancel
          </Button>
          <Button variant="default" onClick={handleButtonSubmit}>
            {loadingCreateWithdraw ? "Creating..." : "Create"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
