import { ColumnDef } from "@tanstack/react-table";
import { Checkbox } from "@/components/ui/checkbox";
import TableActionTransfer from "./table-action";
import { Transfer } from "@/types/model";
import { Link } from "react-router-dom";

export const transferColumns: ColumnDef<Transfer>[] = [
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
    accessorKey: "transfer_from",
    header: "Transfer From",
    cell: ({ row }) => (
      <Link
        to={`/transfers/detail/${row.getValue("transfer_from")}`}
        className="font-mono text-blue-500 underline"
      >
        {row.getValue("transfer_from")}
      </Link>
    ),
  },
  {
    accessorKey: "transfer_to",
    header: "Transfer To",
    cell: ({ row }) => (
      <Link
        to={`/transfers/detail/${row.getValue("transfer_to")}`}
        className="font-mono text-blue-500 underline"
      >
        {row.getValue("transfer_to")}
      </Link>
    ),
  },
  {
    accessorKey: "transfer_amount",
    header: "Transfer Amount",
    cell: ({ row }) => {
      const amount = row.getValue("transfer_amount") as number;
      const formatted = new Intl.NumberFormat("en-US", {
        style: "currency",
        currency: "USD",
      }).format(amount);
      return <div className="text-right font-medium">{formatted}</div>;
    },
  },
  {
    accessorKey: "transfer_time",
    header: "Transfer Time",
    cell: ({ row }) => {
      const rawTime = row.getValue("transfer_time") as string;
      const parsedTime = new Date(rawTime.replace(" +0000 +0000", "Z"));
      return <div>{parsedTime.toLocaleString()}</div>;
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
    cell: ({ row }) => <TableActionTransfer transfer={row.original} />,
  },
];
