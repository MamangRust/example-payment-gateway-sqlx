import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import UpdateUserForm from "../form/UpdateForm";
import useUpdateUser from "@/hooks/admin/user/UpdateUser";

export function EditUser() {
  const {
    formRef,
    editUserId,
    handleButtonSubmit,
    handleSubmit,
    user,
    loadingUpdateUser,
    isModalVisibleEdit,
    showModalEdit,
    hideModalEdit,
  } = useUpdateUser();

  const defaultValues = user
    ? {
        firstname: user.firstname,
        lastname: user.lastname,
        email: user.email,
        password: "",
        confirm_password: "",
      }
    : undefined;



  return (
    <Dialog
      open={isModalVisibleEdit}
      onOpenChange={(open) =>
        open ? showModalEdit(editUserId!) : hideModalEdit()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Edit New User</DialogTitle>
        </DialogHeader>
        <UpdateUserForm
          onSubmit={handleSubmit}
          ref={formRef}
          defaultValues={defaultValues}
        />
        <DialogFooter>
          <Button variant="outline" onClick={hideModalEdit}>
            Cancel
          </Button>
          <Button variant="default" onClick={handleButtonSubmit}>
            {loadingUpdateUser ? "Updating..." : "Update"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
