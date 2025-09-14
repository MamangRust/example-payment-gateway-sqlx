import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogFooter,
  DialogTitle,
} from "@/components/ui/dialog";
import useUpdateCard from "@/hooks/admin/card/UpdateCard";
import UpdateCardForm from "../form/UpdateForm";

export function UpdateCard() {
  const {
    handleSubmit,
    card,
    editCardId,
    loadingUpdateCard,
    isModalVisibleEdit,
    showModalEdit,
    hideModalEdit,
    formRef,
    handleButtonSubmit,
  } = useUpdateCard();

  const defaultValues = card
    ? {
        user_id: {
          value: card.user_id?.toString(),
          label: "Loading...",
        },
        card_type: card.card_type,
        expire_date: card.expire_date,
        cvv: card.cvv,
        card_provider: card.card_provider,
      }
    : undefined;

  return (
    <Dialog
      open={isModalVisibleEdit}
      onOpenChange={(open) =>
        open ? showModalEdit(editCardId!) : hideModalEdit()
      }
    >
      <DialogContent className="max-w-2xl w-full">
        <DialogHeader>
          <DialogTitle>Update Card</DialogTitle>
        </DialogHeader>
        <UpdateCardForm
          onSubmit={handleSubmit}
          ref={formRef}
          defaultValues={defaultValues}
        />
        <DialogFooter>
          <Button variant="outline" onClick={hideModalEdit}>
            Cancel
          </Button>
          <Button variant="default" onClick={handleButtonSubmit}>
            {loadingUpdateCard ? "Updating..." : "Update"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
