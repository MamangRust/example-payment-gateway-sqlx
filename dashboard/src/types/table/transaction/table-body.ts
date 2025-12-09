import { Transaction } from "@/types/model";
import { Table } from "@tanstack/react-table";

export interface TableBodyTransactionProps {
  table: Table<Transaction>;
}
