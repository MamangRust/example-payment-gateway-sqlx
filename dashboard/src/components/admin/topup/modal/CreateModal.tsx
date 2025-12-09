import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import CreateTopupForm from "../form/CreateForm";
import useCreateTopup from "@/hooks/admin/topup/CreateTopup";

export function AddTopup() {
  const {
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingCreateTopup,
    isModalVisible,
    showModal,
    hideModal,
  } = useCreateTopup();

  return (
    <Dialog
      open={isModalVisible}
      onOpenChange={(open) => (open ? showModal() : hideModal())}
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Add New Top-up</DialogTitle>
        </DialogHeader>
        <CreateTopupForm onSubmit={handleSubmit} ref={formRef} />
        <DialogFooter>
          <Button variant="outline" onClick={hideModal}>
            Cancel
          </Button>
          <Button variant="default" onClick={handleButtonSubmit}>
            {loadingCreateTopup ? "Creating..." : "Create"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
