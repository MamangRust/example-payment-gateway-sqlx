import { AddTopup } from "./CreateModal";
import { UpdateTopup } from "./UpdateModal";
import { DeleteTopup } from "./DeleteModal";

export default function TopupModal() {
  return (
    <>
      <AddTopup />
      <UpdateTopup />
      <DeleteTopup />
    </>
  );
}
