import { AddUser } from "./CreateModal";
import { DeleteUser } from "./DeleteModal";
import { EditUser } from "./UpdateModal";

export default function UserModal() {
  return (
    <>
      <AddUser />
      <EditUser />
      <DeleteUser />
    </>
  );
}
