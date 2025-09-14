import { DeletePermanetAllCard } from "./DeleteAllPermanent";
import { DeletePermanetCard } from "./DeletePermanent";
import { RestoreAllCard } from "./RestoreAllModal";
import { RestoreCard } from "./RestoreModal";

export default function CardTrashedModal() {
  return (
    <>
      <DeletePermanetAllCard />
      <DeletePermanetCard />
      <RestoreAllCard />
      <RestoreCard />
    </>
  );
}
