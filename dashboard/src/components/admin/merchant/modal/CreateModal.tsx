import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import CreateMerchantForm from "../form/CreateForm";
import useCreateMerchant from "@/hooks/admin/merchant/CreateMerchant";

export function AddMerchant() {
  const {
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingCreateMerchant,
    isModalVisible,
    showModal,
    hideModal,
  } = useCreateMerchant();

  return (
    <Dialog open={isModalVisible} onOpenChange={showModal}>
      <DialogContent className="max-w-2xl w-full">
        <DialogHeader>
          <DialogTitle>Add New Merchant</DialogTitle>
        </DialogHeader>
        <CreateMerchantForm onSubmit={handleSubmit} ref={formRef} />
        <DialogFooter>
          <Button variant="outline" onClick={hideModal}>
            Cancel
          </Button>
          <Button variant="default" onClick={handleButtonSubmit}>
            {loadingCreateMerchant ? "Creating..." : "Create"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
