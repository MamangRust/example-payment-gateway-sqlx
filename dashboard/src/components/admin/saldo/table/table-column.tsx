import { ColumnDef } from "@tanstack/react-table";
import { Checkbox } from "@/components/ui/checkbox";
import { Saldo } from "@/types/model";
import TableActionSaldo from "./table-action";
import { formatRupiah } from "@/helpers/formatRupiah";

export const saldoColumns: ColumnDef<Saldo>[] = [
  {
    id: "select",
    header: ({ table }) => (
      <Checkbox
        checked={
          table.getIsAllPageRowsSelected() ||
          (table.getIsSomePageRowsSelected() && "indeterminate")
        }
        onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
        aria-label="Select all"
      />
    ),
    cell: ({ row }) => (
      <Checkbox
        checked={row.getIsSelected()}
        onCheckedChange={(value) => row.toggleSelected(!!value)}
        aria-label="Select row"
      />
    ),
    enableSorting: false,
    enableHiding: false,
  },
  {
    accessorKey: "id",
    header: "ID",
    cell: ({ row }) => <div className="font-mono">{row.getValue("id")}</div>,
  },
  {
    accessorKey: "card_number",
    header: "Card Number",
    cell: ({ row }) => (
      <div className="font-mono">{row.getValue("card_number")}</div>
    ),
  },
  {
    accessorKey: "total_balance",
    header: "Total Balance",
    cell: ({ row }) => {
      const totalBalance = row.getValue("total_balance") as number;
      const formatted = totalBalance ? formatRupiah(totalBalance) : "-";
      return <div className="text-right font-medium">{formatted}</div>;
    },
  },
  {
    accessorKey: "withdraw_amount",
    header: "Withdraw Amount",
    cell: ({ row }) => {
      const withdrawAmount = row.getValue("withdraw_amount") as number;
      const formatted = withdrawAmount ? formatRupiah(withdrawAmount) : "-";
      return <div className="text-right">{formatted}</div>;
    },
  },
  {
    accessorKey: "withdraw_time",
    header: "Withdraw Time",
    cell: ({ row }) => {
      const withdrawTime = row.getValue("withdraw_time") as string;
      return <div>{new Date(withdrawTime).toLocaleString()}</div>;
    },
  },
  {
    accessorKey: "created_at",
    header: "Created At",
    cell: ({ row }) => {
      const createdAt = row.getValue("created_at") as string;
      return <div>{new Date(createdAt).toLocaleString()}</div>;
    },
  },
  {
    accessorKey: "updated_at",
    header: "Updated At",
    cell: ({ row }) => {
      const updatedAt = row.getValue("updated_at") as string;
      return <div>{new Date(updatedAt).toLocaleString()}</div>;
    },
  },
  {
    accessorKey: "actions",
    header: () => <div className="text-right">Actions</div>,
    enableSorting: false,
    enableHiding: false,
    cell: ({ row }) => <TableActionSaldo saldo={row.original} />,
  },
];
