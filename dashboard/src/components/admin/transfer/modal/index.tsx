import { AddTransfer } from "./CreateModal";
import { UpdateTransfer } from "./UpdateModal";
import { DeleteTransfer } from "./DeleteModal";

export default function TransferModal() {
  return (
    <>
      <AddTransfer />
      <UpdateTransfer />
      <DeleteTransfer />
    </>
  );
}
