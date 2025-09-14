import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import CreateUserForm from "../form/CreateForm";
import useCreateUser from "@/hooks/admin/user/CreateUser";

export function AddUser() {
  const {
    formRef,
    handleButtonSubmit,
    handleSubmit,
    loadingCreateUser,
    isModalVisible,
    showModal,
    hideModal,
  } = useCreateUser();

  return (
    <Dialog
      open={isModalVisible}
      onOpenChange={(open) => (open ? showModal() : hideModal())}
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Add User</DialogTitle>
        </DialogHeader>
        <CreateUserForm onSubmit={handleSubmit} ref={formRef} />
        <DialogFooter>
          <Button variant="outline" onClick={hideModal}>
            Cancel
          </Button>
          <Button variant="default" onClick={handleButtonSubmit}>
            {loadingCreateUser ? "Creating..." : "Create"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
