import { ColumnDef } from "@tanstack/react-table";
import { Checkbox } from "@/components/ui/checkbox";
import TableActionMerchant from "./table-action";
import { Merchant } from "@/types/model";
import { Link } from "react-router-dom";

export const merchantColumns: ColumnDef<Merchant>[] = [
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
    cell: ({ row }) => (
      <Link
        to={`/merchants/detail/${row.getValue("id")}`}
        className="font-mono text-blue-500 underline"
        title={row.getValue("id")}
      >
        {row.getValue("id")}
      </Link>
    ),
  },
  {
    accessorKey: "name",
    header: "Merchant Name",
    cell: ({ row }) => <div>{row.getValue("name")}</div>,
  },
  {
    accessorKey: "api_key",
    header: "API Key",
    cell: ({ row }) => (
      <Link
        to={`/merchants/detail-key/${row.getValue("api_key")}`}
        className="font-mono text-blue-500 underline"
        title={row.getValue("api_key")}
      >
        {row.getValue("api_key")}
      </Link>
    ),
  },
  {
    accessorKey: "status",
    header: "Status",
    cell: ({ row }) => {
      const status = row.getValue("status") as string;
      const statusStyles =
        status === "active"
          ? "text-green-500 font-medium"
          : status === "inactive"
            ? "text-red-500 font-medium"
            : "text-yellow-500 font-medium";
      return <span className={statusStyles}>{status}</span>;
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
    cell: ({ row }) => <TableActionMerchant merchant={row.original} />,
  },
];
