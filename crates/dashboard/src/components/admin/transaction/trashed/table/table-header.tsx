import { flexRender } from "@tanstack/react-table";
import { TableHeader, TableRow, TableHead } from "@/components/ui/table";
import { TableHeaderTransactionTrashedProps } from "@/types/table";

const TableHeaderTransactionTrashed = ({
  table,
}: TableHeaderTransactionTrashedProps) => (
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

export default TableHeaderTransactionTrashed;
