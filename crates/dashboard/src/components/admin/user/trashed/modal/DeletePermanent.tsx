import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useDeletePermanentUser from "@/hooks/admin/user/trashed/Delete";

export function DeletePermanetUserTrashed() {
  const {
    deletePermanentUserId,
    handleSubmit,
    loadingDeletePermanentUserTrashed,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  } = useDeletePermanentUser();

  return (
    <Dialog
      open={isModalVisibleDeletePermanent}
      onOpenChange={(open) =>
        open
          ? showModalDeletePermanent(deletePermanentUserId!)
          : hideModalDeletePermanent()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Delete User</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to delete permanent this user?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalDeletePermanent}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingDeletePermanentUserTrashed ? "Deleting..." : "Delete"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
