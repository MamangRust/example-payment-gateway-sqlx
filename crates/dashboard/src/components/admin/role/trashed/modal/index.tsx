import { DeletePermanetAllRoleTrashed } from "./DeleteAllPermanent";
import { DeletePermanetRoleTrashed } from "./DeletePermanent";
import { RestoreAllRoleTrashed } from "./RestoreAllModal";
import { RestoreRole } from "./RestoreModal";

export default function RoleTrashedModal() {
  return (
    <>
      <DeletePermanetAllRoleTrashed />
      <DeletePermanetRoleTrashed />
      <RestoreAllRoleTrashed />
      <RestoreRole />
    </>
  );
}
