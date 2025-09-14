import { AddCard} from "./CreateModal"
import { UpdateCard } from "./UpdateModal";
import { DeleteCard } from "./DeleteModal";

export default function CardModal(){
  return(
    <>
      <AddCard />
      <UpdateCard />
      <DeleteCard />
    </>
  )
}
