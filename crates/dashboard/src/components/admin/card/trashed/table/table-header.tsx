import { flexRender } from "@tanstack/react-table";
import { TableHeader, TableRow, TableHead } from "@/components/ui/table";
import { TableHeaderCardTrashedProps } from "@/types/table/card";

const TableHeaderCardTrashed = ({ table }: TableHeaderCardTrashedProps) => (
  <TableHeader>
    {table.getHeaderGroups().map((headerGroup: any) => (
      <TableRow key={headerGroup.id}>
        {headerGroup.headers.map((header: any) => (
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

export default TableHeaderCardTrashed;
