import { ColumnDef } from "@tanstack/react-table";
import { Checkbox } from "@/components/ui/checkbox";
import { MerchantTransaction } from "@/types/model";

export const merchantTransactionColumns: ColumnDef<MerchantTransaction>[] = [
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
    accessorKey: "merchant_name",
    header: "Merchant Name",
    cell: ({ row }) => <div>{row.getValue("merchant_name")}</div>,
  },
  {
    accessorKey: "card_number",
    header: "Card Number",
    cell: ({ row }) => (
      <div className="font-mono truncate" title={row.getValue("card_number")}>
        {row.getValue("card_number")}
      </div>
    ),
  },
  {
    accessorKey: "amount",
    header: "Amount",
    cell: ({ row }) => {
      const amount = row.getValue("amount") as number;
      return <div>${amount.toLocaleString()}</div>;
    },
  },
  {
    accessorKey: "payment_method",
    header: "Payment Method",
    cell: ({ row }) => <div>{row.getValue("payment_method")}</div>,
  },
  {
    accessorKey: "transaction_time",
    header: "Transaction Time",
    cell: ({ row }) => {
      const transactionTime = row.getValue("transaction_time") as string;
      return <div>{new Date(transactionTime).toLocaleString()}</div>;
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
];
