import { AddRole } from "./CreateModal";
import { DeleteRole } from "./DeleteModal";
import { EditRole } from "./UpdateModal";

export default function RoleModal() {
  return (
    <>
      <AddRole />
      <EditRole />
      <DeleteRole />
    </>
  );
}
