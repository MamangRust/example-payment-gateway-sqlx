import { flexRender } from '@tanstack/react-table';
import { TableBody, TableRow, TableCell } from '@/components/ui/table';

const TableBodyWithdraw = ({ table }: any) => (
  <TableBody>
    {table.getRowModel().rows?.length ? (
      table.getRowModel().rows.map((row: any) => (
        <TableRow key={row.id} data-state={row.getIsSelected() && 'selected'}>
          {row.getVisibleCells().map((cell: any) => (
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

export default TableBodyWithdraw;
