import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
} from "@/components/ui/dropdown-menu";
import { Button } from "@/components/ui/button";
import { RefreshCw, Trash2, MoreHorizontal } from "lucide-react";
import { TableActionCardTrashedProps } from "@/types/table";
import useModalCardTrashed from "@/store/card/trashed/modal";

const TableActionCardTrashed = ({ card }: TableActionCardTrashedProps) => {
  const { showModalRestore, showModalDeletePermanent } = useModalCardTrashed();

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost" size="icon" className="h-8 w-8">
          <span className="sr-only">Open menu</span>
          <MoreHorizontal className="h-5 w-5" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end" className="w-48">
        <DropdownMenuLabel>Actions</DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuItem onSelect={() => showModalRestore(card.id)}>
          <RefreshCw className="mr-2 h-4 w-4 text-gray-500" />
          Restore
        </DropdownMenuItem>
        <DropdownMenuItem
          onSelect={() => showModalDeletePermanent(card.id)}
          className="text-red-600"
        >
          <Trash2 className="mr-2 h-4 w-4 text-red-500" />
          Delete Permanently
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};

export default TableActionCardTrashed;
