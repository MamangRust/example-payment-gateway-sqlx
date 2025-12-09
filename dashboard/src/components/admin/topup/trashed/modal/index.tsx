import { DeletePermanetAllTopupTrashed } from "./DeleteAllPermanent";
import { DeletePermanetTopupTrashed } from "./DeletePermanent";
import { RestoreAllTopupTrashed } from "./RestoreAllModal";
import { RestoreTopup } from "./RestoreModal";

export default function TopupTrashedModal() {
  return (
    <>
      <DeletePermanetAllTopupTrashed />
      <DeletePermanetTopupTrashed />
      <RestoreAllTopupTrashed />
      <RestoreTopup />
    </>
  );
}
