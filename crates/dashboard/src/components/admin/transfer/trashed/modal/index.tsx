import { DeletePermanetAllTransferTrashed } from "./DeleteAllPermanent";
import { DeletePermanetTransferTrashed } from "./DeletePermanent";
import { RestoreAllTransferTrashed } from "./RestoreAllModal";
import { RestoreTransfer } from "./RestoreModal";

export default function TransferTrashedModal() {
  return (
    <>
      <DeletePermanetAllTransferTrashed />
      <DeletePermanetTransferTrashed />
      <RestoreAllTransferTrashed />
      <RestoreTransfer />
    </>
  );
}
