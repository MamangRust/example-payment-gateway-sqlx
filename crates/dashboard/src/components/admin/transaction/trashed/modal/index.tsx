import { DeletePermanetAllTransactionTrashed } from "./DeleteAllPermanent";
import { DeletePermanetTransactionTrashed } from "./DeletePermanent";
import { RestoreAllTransactionTrashed } from "./RestoreAllModal";
import { RestoreTransaction } from "./RestoreModal";

export default function TransactionTrashedModal() {
  return (
    <>
      <DeletePermanetAllTransactionTrashed />
      <DeletePermanetTransactionTrashed />
      <RestoreAllTransactionTrashed />
      <RestoreTransaction />
    </>
  );
}
