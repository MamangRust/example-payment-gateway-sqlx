import { flexRender } from "@tanstack/react-table";
import { TableHeader, TableRow, TableHead } from "@/components/ui/table";
import { TableHeaderMerchantTransactionProps } from "@/types/table";

const TableHeaderMerchantTransaction = ({
  table,
}: TableHeaderMerchantTransactionProps) => (
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

export default TableHeaderMerchantTransaction;
