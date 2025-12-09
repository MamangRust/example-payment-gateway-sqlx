import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import CreateSaldoForm from "../form/CreateForm";
import useCreateSaldo from "@/hooks/admin/saldo/CreateSaldo";

export function AddSaldo() {
  const {
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingCreateSaldo,
    isModalVisible,
    showModal,
    hideModal,
  } = useCreateSaldo();

  return (
    <Dialog
      open={isModalVisible}
      onOpenChange={(open) => (open ? showModal() : hideModal())}
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Add New Card</DialogTitle>
        </DialogHeader>
        <CreateSaldoForm onSubmit={handleSubmit} ref={formRef} />
        <DialogFooter>
          <Button variant="outline" onClick={hideModal}>
            Cancel
          </Button>
          <Button variant="default" onClick={handleButtonSubmit}>
            {loadingCreateSaldo ? "Creating..." : "Create"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
