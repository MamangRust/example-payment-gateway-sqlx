import { flexRender } from "@tanstack/react-table";
import { TableHeader, TableRow, TableHead } from "@/components/ui/table";
import { TableHeaderUserProps } from "@/types/table/user/table-header";

const TableHeaderUser = ({ table }: TableHeaderUserProps) => (
  <TableHeader>
    {table.getHeaderGroups().map((headerGroup) => (
      <TableRow key={headerGroup.id}>
        {headerGroup.headers.map((header) => (
          <TableHead key={header.id}>
            {header.isPlaceholder
              ? null
              : flexRender(header.column.columnDef.header, header.getContext())}
          </TableHead>
        ))}
      </TableRow>
    ))}
  </TableHeader>
);

export default TableHeaderUser;
