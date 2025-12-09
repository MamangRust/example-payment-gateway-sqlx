import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
} from "@/components/ui/dropdown-menu";
import { Button } from "@/components/ui/button";
import { Pencil, Trash, MoreHorizontal } from "lucide-react";
import { TableActionTopupProps } from "@/types/table";
import useModalTopup from "@/store/topup/modal";

const TableActionTopup = ({ topup }: TableActionTopupProps) => {
  const { showModalEdit, showModalDelete } = useModalTopup();

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
        <DropdownMenuItem onSelect={() => showModalEdit(topup.id)}>
          <Pencil className="mr-2 h-4 w-4 text-gray-500" />
          Edit
        </DropdownMenuItem>
        <DropdownMenuItem
          onSelect={() => showModalDelete(topup.id)}
          className="text-red-600"
        >
          <Trash className="mr-2 h-4 w-4 text-red-500" />
          Delete
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};

export default TableActionTopup;
