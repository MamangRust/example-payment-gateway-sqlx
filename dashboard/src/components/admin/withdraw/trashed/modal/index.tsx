import { DeletePermanetAllWithdrawTrashed } from "./DeleteAllPermanent";
import { DeletePermanetWithdrawTrashed } from "./DeletePermanent";
import { RestoreAllWithdrawTrashed } from "./RestoreAllModal";
import { RestoreWithdraw } from "./RestoreModal";

export default function WithdrawTrashedModal() {
  return (
    <>
      <DeletePermanetAllWithdrawTrashed />
      <DeletePermanetWithdrawTrashed />
      <RestoreAllWithdrawTrashed />
      <RestoreWithdraw />
    </>
  );
}
