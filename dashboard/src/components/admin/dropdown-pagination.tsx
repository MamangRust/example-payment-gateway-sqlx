import { DropdownMenu } from "@radix-ui/react-dropdown-menu";
import {
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "../ui/dropdown-menu";

const PaginationDropdown = ({ pagination, setPagination, table }: any) => {
  return (
    <div className="flex items-center space-x-2">
      <span className="text-sm">Rows per page:</span>

      <DropdownMenu>
        <DropdownMenuTrigger className="border rounded px-2 py-1 text-sm">
          {pagination.pageSize}
        </DropdownMenuTrigger>

        <DropdownMenuContent className="p-2">
          {[5, 10, 20, 50, 100].map((size) => (
            <DropdownMenuItem
              key={size}
              className="px-4 py-2 hover:bg-gray-200 cursor-pointer"
              onClick={() => {
                setPagination(size);
                table.setPageSize(size);
              }}
            >
              {size}
            </DropdownMenuItem>
          ))}
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
};

export default PaginationDropdown;
