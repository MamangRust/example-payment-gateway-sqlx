import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useDeletePermanentAllUser from "@/hooks/admin/user/trashed/DeleteAll";

export function DeletePermanetAllUserTrashed() {
  const {
    handleSubmit,
    loadingDeletePermanentAllUserTrashed,
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  } = useDeletePermanentAllUser();

  return (
    <Dialog
      open={isModalVisibleDeletePermanentAll}
      onOpenChange={(open) =>
        open ? showModalDeletePermanentAll() : hideModalDeletePermanentAll()
      }
    >
      <DialogContent className="max-w-md w-full">
        <DialogHeader>
          <DialogTitle>Delete User All</DialogTitle>
        </DialogHeader>
        <div className="text-sm text-gray-600">
          Are you sure you want to delete permanent all this user?
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={hideModalDeletePermanentAll}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={handleSubmit}>
            {loadingDeletePermanentAllUserTrashed ? "Deleting..." : "Delete"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
