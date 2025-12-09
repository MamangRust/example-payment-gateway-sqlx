import { DeletePermanetAllUserTrashed } from "./DeleteAllPermanent";
import { DeletePermanetUserTrashed } from "./DeletePermanent";
import { RestoreAllUserTrashed } from "./RestoreAllModal";
import { RestoreUser } from "./RestoreModal";

export default function UserTrashedModal() {
  return (
    <>
      <DeletePermanetAllUserTrashed />
      <DeletePermanetUserTrashed />
      <RestoreAllUserTrashed />
      <RestoreUser />
    </>
  );
}
