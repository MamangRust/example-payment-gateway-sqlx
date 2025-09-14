import { ColumnDef } from "@tanstack/react-table";
import { Checkbox } from "@/components/ui/checkbox";
import TableActionWithdraw from "./table-action";
import { WithdrawTrashed } from "@/types/model";

export const withdrawTrashedColumns: ColumnDef<WithdrawTrashed>[] = [
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
    accessorKey: "withdraw_amount",
    header: "Withdraw Amount",
    cell: ({ row }) => {
      const amount = row.getValue("withdraw_amount") as number;
      const formatted = new Intl.NumberFormat("en-US", {
        style: "currency",
        currency: "USD",
      }).format(amount);
      return <div className="text-right font-medium">{formatted}</div>;
    },
  },
  {
    accessorKey: "withdraw_time",
    header: "Withdraw Time",
    cell: ({ row }) => {
      const time = row.getValue("withdraw_time") as string;
      return <div>{new Date(time).toLocaleString()}</div>;
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
    cell: ({ row }) => <TableActionWithdraw withdraw={row.original} />,
  },
];
