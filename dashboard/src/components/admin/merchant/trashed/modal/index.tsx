import { DeletePermanetAllMerchant } from "./DeleteAllPermanent";
import { DeletePermanetMerchant } from "./DeletePermanent";
import { RestoreAllMerchant } from "./RestoreAllModal";
import { RestoreMerchant } from "./RestoreModal";

export default function MerchantTrashedModal() {
  return (
    <>
      <DeletePermanetAllMerchant />
      <DeletePermanetMerchant />
      <RestoreAllMerchant />
      <RestoreMerchant />
    </>
  );
}
