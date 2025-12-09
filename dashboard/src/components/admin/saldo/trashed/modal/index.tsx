import { DeletePermanetAllSaldo } from "./DeleteAllPermanent";
import { DeletePermanetSaldo } from "./DeletePermanent";
import { RestoreAllSaldo } from "./RestoreAllModal";
import { RestoreSaldo } from "./RestoreModal";

export default function SaldoTrashedModal() {
  return (
    <>
      <DeletePermanetAllSaldo />
      <DeletePermanetSaldo />
      <RestoreAllSaldo />
      <RestoreSaldo />
    </>
  );
}
