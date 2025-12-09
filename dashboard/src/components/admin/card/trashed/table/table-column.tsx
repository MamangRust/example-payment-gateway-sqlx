import { ColumnDef } from "@tanstack/react-table";
import { Checkbox } from "@/components/ui/checkbox";
import TableActionCard from "./table-action";
import { CardTrashed } from "@/types/model/card";

export const cardTrashedColumns: ColumnDef<CardTrashed>[] = [
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
    accessorKey: "card_type",
    header: "Card Type",
    cell: ({ row }) => <div>{row.getValue("card_type")}</div>,
  },
  {
    accessorKey: "expire_date",
    header: "Expire Date",
    cell: ({ row }) => {
      const expireDate = row.getValue("expire_date") as string;
      return <div>{new Date(expireDate).toLocaleDateString()}</div>;
    },
  },
  {
    accessorKey: "card_provider",
    header: "Provider",
    cell: ({ row }) => <div>{row.getValue("card_provider")}</div>,
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
    cell: ({ row }) => <TableActionCard card={row.original} />,
  },
];
