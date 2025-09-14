import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import CreateTransferForm from "../form/CreateForm";
import useCreateTransfer from "@/hooks/admin/transfer/CreateTransfer";

export function AddTransfer() {
  const {
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingCreateTransfer,
    isModalVisible,
    showModal,
    hideModal,
  } = useCreateTransfer();

  return (
    <Dialog
      open={isModalVisible}
      onOpenChange={(open) => (open ? showModal() : hideModal())}
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Add New Transfer</DialogTitle>
        </DialogHeader>
        <CreateTransferForm onSubmit={handleSubmit} ref={formRef} />
        <DialogFooter>
          <Button variant="outline" onClick={hideModal}>
            Cancel
          </Button>
          <Button variant="default" onClick={handleButtonSubmit}>
            {loadingCreateTransfer ? "Creating..." : "Create"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
