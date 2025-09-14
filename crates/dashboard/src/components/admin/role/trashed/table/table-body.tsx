import { flexRender } from "@tanstack/react-table";
import { TableBody, TableRow, TableCell } from "@/components/ui/table";
import { TableBodyRoleTrashedProps } from "@/types/table/role";

const TableBodyRoleTrashed = ({ table }: TableBodyRoleTrashedProps) => (
  <TableBody>
    {table.getRowModel().rows?.length ? (
      table.getRowModel().rows.map((row) => (
        <TableRow key={row.id} data-state={row.getIsSelected() && "selected"}>
          {row.getVisibleCells().map((cell) => (
            <TableCell key={cell.id}>
              {flexRender(cell.column.columnDef.cell, cell.getContext())}
            </TableCell>
          ))}
        </TableRow>
      ))
    ) : (
      <TableRow>
        <TableCell
          colSpan={table.getAllColumns().length}
          className="h-24 text-center"
        >
          No results.
        </TableCell>
      </TableRow>
    )}
  </TableBody>
);

export default TableBodyRoleTrashed;
