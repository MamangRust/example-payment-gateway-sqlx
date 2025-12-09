import { AddTransaction } from "./CreateModal";
import { UpdateTransaction } from "./UpdateModal";
import { DeleteTransaction } from "./DeleteModal";

export default function TransactionModal() {
  return (
    <>
      <AddTransaction />
      <UpdateTransaction />
      <DeleteTransaction />
    </>
  );
}
