import { AddMerchant } from "./CreateModal";
import { UpdateMerchant } from "./UpdateModal";
import { DeleteMerchant } from "./DeleteModal";

export default function MerchantModal() {
  return (
    <>
      <AddMerchant />
      <UpdateMerchant />
      <DeleteMerchant />
    </>
  );
}
