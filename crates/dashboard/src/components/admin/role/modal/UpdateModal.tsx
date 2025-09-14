import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import UpdateRoleForm from "../form/UpdateForm";
import useUpdateRole from "@/hooks/admin/role/UpdateRole";

export function EditRole() {
  const {
    formRef,
    editRoleId,
    handleButtonSubmit,
    handleSubmit,
    Role,
    loadingUpdateRole,
    isModalVisibleEdit,
    showModalEdit,
    hideModalEdit,
  } = useUpdateRole();

  const defaultValues = Role
    ? {
        name: Role.name,
      }
    : undefined;

  return (
    <Dialog
      open={isModalVisibleEdit}
      onOpenChange={(open) =>
        open ? showModalEdit(editRoleId!) : hideModalEdit()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Edit New Role</DialogTitle>
        </DialogHeader>
        <UpdateRoleForm
          onSubmit={handleSubmit}
          ref={formRef}
          defaultValues={defaultValues}
        />
        <DialogFooter>
          <Button variant="outline" onClick={hideModalEdit}>
            Cancel
          </Button>
          <Button variant="default" onClick={handleButtonSubmit}>
            {loadingUpdateRole ? "Updating..." : "Update"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
